mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use std::{env, fs, io::{self, Write}, time::Instant};

use interpreter::Interpreter;
use lexer::lex;
use parser::Parser;

const RESET: &str = "\x1b[0m";
const PURPLE: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const GREEN: &str = "\x1b[92m";
const RED: &str = "\x1b[91m";
const DIM: &str = "\x1b[90m";
const BOLD: &str = "\x1b[1m";

fn banner() {
    println!("{PURPLE}{BOLD}╭──────────────────────────────────────────────╮{RESET}");
    println!("{PURPLE}{BOLD}│                 N+ 1.0.0                    │{RESET}");
    println!("{PURPLE}{BOLD}│          Build anything. Simply.            │{RESET}");
    println!("{PURPLE}{BOLD}╰──────────────────────────────────────────────╯{RESET}");
}

fn help() {
    banner();
    println!();
    println!("{BOLD}USAGE{RESET}");
    println!("  nplus run <file.npl>      Run a program");
    println!("  nplus check <file.npl>    Validate a program");
    println!("  nplus tokens <file.npl>   Show lexer tokens");
    println!("  nplus repl                Start the N+ REPL");
    println!("  nplus web <file.npl>      Run an N+ Web app");
    println!("  nplus help                Show this help");
    println!();
    println!("{DIM}N+ is developed by Netrix Development.{RESET}");
}

fn read_and_parse(path: &str) -> Result<Vec<ast::Stmt>, String> {
    if !path.ends_with(".npl") {
        return Err(format!("NPL0001: `{path}` is not an N+ source file. Expected `.npl`."));
    }

    let source = fs::read_to_string(path)
        .map_err(|e| format!("NPL0002: cannot read `{path}`: {e}"))?;

    let tokens = lex(&source).map_err(|e| {
        format!(
            "NPL1001: line {}, column {}: {}",
            e.line, e.column, e.message
        )
    })?;

    Parser::new(tokens)
        .parse()
        .map_err(|e| format!("NPL1100: {}" , e.message))
}

fn format_error(error: &str) {
    println!();
    println!("{RED}{BOLD}✖ N+ encountered an error{RESET}");
    println!("{DIM}────────────────────────────────────────────────{RESET}");
    println!("{RED}{error}{RESET}");
    println!("{DIM}Hint: run `nplus check <file.npl>` before running again.{RESET}");
}

fn run_file(path: &str) -> i32 {
    let start = Instant::now();
    println!("{CYAN}{BOLD}▶ Running{RESET} {BOLD}{path}{RESET}");
    println!("{DIM}────────────────────────────────────────────────{RESET}");

    match read_and_parse(path) {
        Ok(program) => {
            let mut vm = Interpreter::new();
            match vm.run(&program) {
                Ok(()) => {
                    println!("{DIM}────────────────────────────────────────────────{RESET}");
                    println!(
                        "{GREEN}✓ Finished{RESET}  {DIM}{} ms{RESET}",
                        start.elapsed().as_secs_f64() * 1000.0
                    );
                    0
                }
                Err(error) => {
                    format_error(&error);
                    1
                }
            }
        }
        Err(error) => {
            format_error(&error);
            1
        }
    }
}

fn check_file(path: &str) -> i32 {
    match read_and_parse(path) {
        Ok(_) => {
            println!("{GREEN}{BOLD}✓ Valid N+ source{RESET}  {DIM}{path}{RESET}");
            0
        }
        Err(error) => {
            format_error(&error);
            1
        }
    }
}

fn tokens(path: &str) -> i32 {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(e) => {
            format_error(&format!("NPL0002: cannot read `{path}`: {e}"));
            return 1;
        }
    };

    match lex(&source) {
        Ok(tokens) => {
            for token in tokens {
                println!("{CYAN}{token:?}{RESET}");
            }
            0
        }
        Err(error) => {
            format_error(&format!(
                "NPL1001: line {}, column {}: {}",
                error.line, error.column, error.message
            ));
            1
        }
    }
}

fn repl() {
    banner();
    println!("{DIM}Type `exit` to leave. Statements end with `*` or `!`.{RESET}");
    println!();

    let mut vm = Interpreter::new();

    loop {
        print!("{PURPLE}nplus{RESET} {DIM}›{RESET} ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }

        if line.trim() == "exit" {
            println!("{DIM}Goodbye.{RESET}");
            break;
        }

        match lex(&line).and_then(|tokens| {
            Parser::new(tokens).parse().map_err(|e| lexer::LexError {
                line: 1,
                column: 1,
                message: e.message,
            })
        }) {
            Ok(program) => {
                if let Err(error) = vm.run(&program) {
                    format_error(&error);
                }
            }
            Err(error) => format_error(&format!(
                "NPL1100: line {}, column {}: {}",
                error.line, error.column, error.message
            )),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let code = match args[1].as_str() {
        "help" | "--help" | "-h" => {
            help();
            0
        }
        "run" => {
            if args.len() < 3 {
                format_error("NPL0004: missing source file.");
                1
            } else {
                run_file(&args[2])
            }
        }
        "check" => {
            if args.len() < 3 {
                format_error("NPL0004: missing source file.");
                1
            } else {
                check_file(&args[2])
            }
        }
        "tokens" => {
            if args.len() < 3 {
                format_error("NPL0004: missing source file.");
                1
            } else {
                tokens(&args[2])
            }
        }
        "repl" => {
            repl();
            0
        }
        "web" => {
            if args.len() < 3 {
                format_error("NPL0004: missing N+ Web source file.");
                1
            } else {
                run_file(&args[2])
            }
        }
        unknown => {
            format_error(&format!("NPL0005: unknown command `{unknown}`."));
            1
        }
    };

    if code != 0 {
        std::process::exit(code);
    }
}
