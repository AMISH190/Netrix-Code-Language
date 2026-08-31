use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

use crate::ast::{Expr, Stmt};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Range(i64, i64),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => {
                if number.fract() == 0.0 {
                    write!(f, "{number:.0}")
                } else {
                    write!(f, "{number}")
                }
            }
            Value::String(string) => write!(f, "{string}"),
            Value::Bool(boolean) => write!(f, "{boolean}"),
            Value::Null => write!(f, "null"),
            Value::Range(start, end) => write!(f, "{start}..{end}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Function {
    params: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
struct WebRoute {
    method: String,
    path: String,
    response: String,
}

#[derive(Debug, Clone)]
struct WebState {
    host: String,
    port: u16,
    routes: Vec<WebRoute>,
    static_dir: Option<PathBuf>,
}

impl WebState {
    fn new() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 3000,
            routes: Vec::new(),
            static_dir: None,
        }
    }

    fn add_route(&mut self, method: &str, path: String, response: String) {
        self.routes.retain(|r| !(r.method == method && r.path == path));
        self.routes.push(WebRoute {
            method: method.into(),
            path,
            response,
        });
    }
}

pub struct Interpreter {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, Function>,
    web: WebState,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            web: WebState::new(),
        }
    }

    pub fn run(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for statement in statements {
            if self.exec(statement)?.is_some() {
                return Err("NPL2010: `return` can only be used inside a function".into());
            }
        }
        Ok(())
    }

    fn exec(&mut self, statement: &Stmt) -> Result<Option<Value>, String> {
        match statement {
            Stmt::Let {
                name,
                value,
                mutable: _,
            } => {
                let value = self.eval(value)?;
                self.vars.insert(name.clone(), value);
                Ok(None)
            }

            Stmt::Assign { name, value } => {
                if !self.vars.contains_key(name) {
                    return Err(format!(
                        "NPL2001: variable `{name}` does not exist"
                    ));
                }

                let value = self.eval(value)?;
                self.vars.insert(name.clone(), value);
                Ok(None)
            }

            Stmt::Expr(expression) => {
                self.eval(expression)?;
                Ok(None)
            }

            Stmt::Print(expression) => {
                println!("{}", self.eval(expression)?);
                Ok(None)
            }

            Stmt::Return(expression) => {
                let value = match expression {
                    Some(expression) => self.eval(expression)?,
                    None => Value::Null,
                };
                Ok(Some(value))
            }

            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition_value = self.eval(condition)?;
                let branch = if truthy(&condition_value) {
                    then_branch
                } else {
                    else_branch
                };

                for statement in branch {
                    if let Some(value) = self.exec(statement)? {
                        return Ok(Some(value));
                    }
                }

                Ok(None)
            }

            Stmt::While { condition, body } => {
                let mut guard = 0usize;

                loop {
                    let condition_value = self.eval(condition)?;
                    if !truthy(&condition_value) {
                        break;
                    }

                    for statement in body {
                        if let Some(value) = self.exec(statement)? {
                            return Ok(Some(value));
                        }
                    }

                    // Protect beginners from an accidental endless loop
                    // while keeping a very large practical limit.
                    guard += 1;
                    if guard > 10_000_000 {
                        return Err(
                            "NPL2011: while loop exceeded 10,000,000 iterations".into(),
                        );
                    }
                }

                Ok(None)
            }

            Stmt::For {
                name,
                iterable,
                body,
            } => {
                let value = self.eval(iterable)?;

                match value {
                    Value::Range(start, end) => {
                        let step = if start <= end { 1 } else { -1 };
                        let mut current = start;

                        while if step > 0 {
                            current < end
                        } else {
                            current > end
                        } {
                            self.vars
                                .insert(name.clone(), Value::Number(current as f64));

                            for statement in body {
                                if let Some(value) = self.exec(statement)? {
                                    return Ok(Some(value));
                                }
                            }

                            current += step;
                        }

                        Ok(None)
                    }
                    _ => Err(
                        "NPL2012: `for` currently expects a numeric range such as `1..10`".into(),
                    ),
                }
            }

            Stmt::Function {
                name,
                params,
                body,
            } => {
                self.funcs.insert(
                    name.clone(),
                    Function {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Ok(None)
            }
        }
    }

    fn eval(&mut self, expression: &Expr) -> Result<Value, String> {
        match expression {
            Expr::Number(number) => Ok(Value::Number(*number)),
            Expr::String(string) => Ok(Value::String(interpolate(string, &self.vars))),
            Expr::Bool(boolean) => Ok(Value::Bool(*boolean)),
            Expr::Identifier(name) => self
                .vars
                .get(name)
                .cloned()
                .ok_or_else(|| format!("NPL2002: unknown variable `{name}`")),

            Expr::Range { start, end } => {
                let start_value = self.eval(start)?;
                let end_value = self.eval(end)?;

                let start_number = number_value(&start_value)
                    .ok_or_else(|| "NPL2013: range start must be a number".to_string())?;
                let end_number = number_value(&end_value)
                    .ok_or_else(|| "NPL2014: range end must be a number".to_string())?;

                Ok(Value::Range(start_number as i64, end_number as i64))
            }

            Expr::Unary { op, expr } => {
                let value = self.eval(expr)?;

                match (op.as_str(), value) {
                    ("-", Value::Number(number)) => Ok(Value::Number(-number)),
                    ("not", value) => Ok(Value::Bool(!truthy(&value))),
                    _ => Err("NPL2003: invalid unary operation".into()),
                }
            }

            Expr::Binary { left, op, right } => {
                let left_value = self.eval(left)?;
                let right_value = self.eval(right)?;
                binary(left_value, op, right_value)
            }

            Expr::Call { name, args } => self.eval_call(name, args),
        }
    }

    fn eval_call(&mut self, name: &str, args: &[Expr]) -> Result<Value, String> {
        if name == "print" {
            let mut values = Vec::new();
            for argument in args {
                values.push(self.eval(argument)?.to_string());
            }
            println!("{}", values.join(" "));
            return Ok(Value::Null);
        }

        match name {
            "web.listen" => {
                let port = require_port(args, self)?;
                self.web.port = port;
                println!("N+ Web: configured http://127.0.0.1:{port}");
                return Ok(Value::Null);
            }

            "web.get" => {
                if args.len() != 2 {
                    return Err("NPL3002: `web.get` expects `(path, body)`".into());
                }

                let path = self.eval(&args[0])?;
                let body = self.eval(&args[1])?;
                let path = match path {
                    Value::String(value) => value,
                    _ => return Err("NPL3003: web route path must be a string".into()),
                };
                let body = body.to_string();

                self.web.add_route("GET", path, body);
                return Ok(Value::Null);
            }

            "web.start" => {
                return self.start_web_server();
            }

            "web.serve" => {
                return self.serve_static(args);
            }

            "web.html" => {
                if args.len() != 1 {
                    return Err("NPL3004: `web.html` expects `(html)`".into());
                }
                return Ok(self.eval(&args[0])?);
            }

            _ => {}
        }

        let function = self
            .funcs
            .get(name)
            .cloned()
            .ok_or_else(|| format!("NPL2004: unknown function `{name}`"))?;

        if function.params.len() != args.len() {
            return Err(format!(
                "NPL2005: function `{name}` expects {} arguments, got {}",
                function.params.len(),
                args.len()
            ));
        }

        // Evaluate arguments before mutating the call scope.
        let mut values = Vec::with_capacity(args.len());
        for argument in args {
            values.push(self.eval(argument)?);
        }

        let saved_vars = self.vars.clone();

        for (parameter, value) in function.params.iter().zip(values.into_iter()) {
            self.vars.insert(parameter.clone(), value);
        }

        let mut result = Value::Null;

        for statement in &function.body {
            if let Some(value) = self.exec(statement)? {
                result = value;
                break;
            }
        }

        self.vars = saved_vars;
        Ok(result)
    }

    fn start_web_server(&mut self) -> Result<Value, String> {
        let port = self.web.port;
        let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|error| {
            format!("NPL3005: could not start N+ Web on port {port}: {error}")
        })?;

        println!();
        println!("N+ Web");
        println!("────────────────────────────────────────────────");
        println!("✓ Server running at http://127.0.0.1:{port}");
        println!("✓ Press Ctrl+C to stop the server.");
        println!();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(error) = handle_web_request(stream, &self.web.routes) {
                        eprintln!("N+ Web request error: {error}");
                    }
                }
                Err(error) => {
                    eprintln!("N+ Web connection error: {error}");
                }
            }
        }

        Ok(Value::Null)
    }

    fn serve_static(&mut self, args: &[Expr]) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("NPL3006: `web.serve` expects `(directory, port)`".into());
        }

        let directory = match self.eval(&args[0])? {
            Value::String(path) => path,
            _ => return Err("NPL3007: web directory must be a string".into()),
        };

        let port_value = self.eval(&args[1])?;
        let port = number_value(&port_value)
            .ok_or_else(|| "NPL3008: web port must be a number".to_string())? as u16;

        let root = Path::new(&directory).canonicalize().map_err(|error| {
            format!("NPL3009: cannot open web directory `{directory}`: {error}")
        })?;

        let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|error| {
            format!("NPL3010: could not start N+ Web on port {port}: {error}")
        })?;

        println!();
        println!("N+ Web");
        println!("────────────────────────────────────────────────");
        println!("✓ Serving {}", root.display());
        println!("✓ http://127.0.0.1:{port}");
        println!("✓ Press Ctrl+C to stop the server.");
        println!();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(error) = handle_static_request(stream, &root) {
                        eprintln!("N+ Web request error: {error}");
                    }
                }
                Err(error) => {
                    eprintln!("N+ Web connection error: {error}");
                }
            }
        }

        Ok(Value::Null)
    }
}

fn require_port(args: &[Expr], interpreter: &mut Interpreter) -> Result<u16, String> {
    if args.len() != 1 {
        return Err("NPL3001: `web.listen` expects `(port)`".into());
    }

    let value = interpreter.eval(&args[0])?;
    let port = number_value(&value)
        .ok_or_else(|| "NPL3001: web port must be a number".to_string())?;

    if !(1.0..=65535.0).contains(&port) {
        return Err("NPL3011: web port must be between 1 and 65535".into());
    }

    Ok(port as u16)
}

fn handle_web_request(
    mut stream: TcpStream,
    routes: &[WebRoute],
) -> Result<(), String> {
    let mut buffer = [0u8; 8192];
    let bytes_read = stream
        .read(&mut buffer)
        .map_err(|error| error.to_string())?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or_default();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    if method != "GET" {
        return write_response(
            &mut stream,
            405,
            "Method Not Allowed",
            "N+ Web currently supports GET routes.",
            "text/plain; charset=utf-8",
        );
    }

    let route = routes.iter().find(|route| route.path == path);

    match route {
        Some(route) => write_response(
            &mut stream,
            200,
            "OK",
            &route.body,
            "text/html; charset=utf-8",
        ),
        None => write_response(
            &mut stream,
            404,
            "Not Found",
            "<h1>404</h1><p>N+ Web route not found.</p>",
            "text/html; charset=utf-8",
        ),
    }
}

fn handle_static_request(
    mut stream: TcpStream,
    root: &Path,
) -> Result<(), String> {
    let mut buffer = [0u8; 8192];
    let bytes_read = stream
        .read(&mut buffer)
        .map_err(|error| error.to_string())?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request.lines().next().unwrap_or_default();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let raw_path = parts.next().unwrap_or("/");

    if method != "GET" {
        return write_response(
            &mut stream,
            405,
            "Method Not Allowed",
            "N+ Web currently supports GET requests.",
            "text/plain; charset=utf-8",
        );
    }

    let clean_path = raw_path.split('?').next().unwrap_or("/");
    let relative = clean_path.trim_start_matches('/');
    let candidate = if relative.is_empty() {
        root.join("index.html")
    } else {
        root.join(relative)
    };

    let file_path = candidate.canonicalize().ok();
    let safe_path = match file_path {
        Some(path) if path.starts_with(root) => path,
        _ => {
            return write_response(
                &mut stream,
                404,
                "Not Found",
                "<h1>404</h1><p>File not found.</p>",
                "text/html; charset=utf-8",
            )
        }
    };

    if safe_path.is_dir() {
        let index = safe_path.join("index.html");
        if index.exists() {
            return serve_file(&mut stream, &index, 200, "OK");
        }
    }

    if !safe_path.exists() {
        return write_response(
            &mut stream,
            404,
            "Not Found",
            "<h1>404</h1><p>File not found.</p>",
            "text/html; charset=utf-8",
        );
    }

    serve_file(&mut stream, &safe_path, 200, "OK")
}

fn serve_file(
    stream: &mut TcpStream,
    path: &Path,
    status: u16,
    reason: &str,
) -> Result<(), String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    let content_type = content_type_for(path);

    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        bytes.len()
    );

    stream
        .write_all(header.as_bytes())
        .map_err(|error| error.to_string())?;
    stream.write_all(&bytes).map_err(|error| error.to_string())?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    body: &str,
    content_type: &str,
) -> Result<(), String> {
    let bytes = body.as_bytes();
    let header = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        bytes.len()
    );

    stream
        .write_all(header.as_bytes())
        .map_err(|error| error.to_string())?;
    stream.write_all(bytes).map_err(|error| error.to_string())?;
    Ok(())
}

fn content_type_for(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") | Some("htm") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("ico") => "image/x-icon",
        Some("txt") => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

fn interpolate(string: &str, variables: &HashMap<String, Value>) -> String {
    let mut output = string.to_string();

    for (name, value) in variables {
        output = output.replace(
            &format!("{{{name}}}"),
            &value.to_string(),
        );
    }

    output
}

fn number_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => Some(*number),
        _ => None,
    }
}

fn truthy(value: &Value) -> bool {
    match value {
        Value::Bool(boolean) => *boolean,
        Value::Number(number) => *number != 0.0,
        Value::String(string) => !string.is_empty(),
        Value::Null => false,
        Value::Range(start, end) => start != end,
    }
}

fn binary(left: Value, operator: &str, right: Value) -> Result<Value, String> {
    match operator {
        "+" => match (left, right) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(format!("{x}{y}"))),
            (x, y) => Ok(Value::String(format!("{x}{y}"))),
        },
        "-" => num(left, right, |x, y| x - y),
        "*" => num(left, right, |x, y| x * y),
        "/" => num(left, right, |x, y| x / y),
        "%" => num(left, right, |x, y| x % y),
        "==" => Ok(Value::Bool(equals(&left, &right))),
        "!=" => Ok(Value::Bool(!equals(&left, &right))),
        ">" => cmp(left, right, |x, y| x > y),
        ">=" => cmp(left, right, |x, y| x >= y),
        "<" => cmp(left, right, |x, y| x < y),
        "<=" => cmp(left, right, |x, y| x <= y),
        "and" => Ok(Value::Bool(truthy(&left) && truthy(&right))),
        "or" => Ok(Value::Bool(truthy(&left) || truthy(&right))),
        _ => Err(format!("NPL2006: unsupported operator `{operator}`")),
    }
}

fn equals(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Null, Value::Null) => true,
        _ => left.to_string() == right.to_string(),
    }
}

fn num(
    left: Value,
    right: Value,
    operation: fn(f64, f64) -> f64,
) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Number(operation(x, y))),
        _ => Err("NPL2007: numeric operator needs numbers".into()),
    }
}

fn cmp(
    left: Value,
    right: Value,
    operation: fn(f64, f64) -> bool,
) -> Result<Value, String> {
    match (left, right) {
        (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(operation(x, y))),
        _ => Err("NPL2008: comparison needs numbers".into()),
    }
}
