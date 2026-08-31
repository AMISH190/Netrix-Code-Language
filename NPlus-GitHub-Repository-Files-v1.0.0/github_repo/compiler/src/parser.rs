use crate::ast::{Expr, Stmt};
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
        }
        token
    }

    fn check(&self, token: &Token) -> bool {
        self.peek() == token
    }

    fn consume(&mut self, token: &Token, message: &str) -> Result<(), ParseError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                message: message.into(),
            })
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.check(&Token::EOF) {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Token::Let | Token::Var | Token::Const => self.var_decl(),
            Token::Fn => self.fn_decl(),
            Token::If => self.if_stmt(),
            Token::While => self.while_stmt(),
            Token::For => self.for_stmt(),
            Token::Return => {
                self.advance();
                let expression = if self.check(&Token::Terminator) {
                    None
                } else {
                    Some(self.expression()?)
                };
                self.terminator()?;
                Ok(Stmt::Return(expression))
            }
            _ => {
                if let Token::Identifier(name) = self.peek().clone() {
                    if self.current + 1 < self.tokens.len()
                        && self.tokens[self.current + 1] == Token::Equal
                    {
                        self.advance();
                        self.advance();
                        let value = self.expression()?;
                        self.terminator()?;
                        return Ok(Stmt::Assign { name, value });
                    }
                }

                let expression = self.expression()?;
                self.terminator()?;
                Ok(Stmt::Expr(expression))
            }
        }
    }

    fn var_decl(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.advance();
        let mutable = !matches!(keyword, Token::Const);

        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParseError {
                    message: "Expected variable name".into(),
                })
            }
        };

        self.consume(&Token::Equal, "Expected `=` after variable name")?;
        let value = self.expression()?;
        self.terminator()?;

        Ok(Stmt::Let {
            name,
            mutable,
            value,
        })
    }

    fn fn_decl(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParseError {
                    message: "Expected function name".into(),
                })
            }
        };

        self.consume(
            &Token::LeftParen,
            "Expected `(` after function name",
        )?;

        let mut params = Vec::new();

        if !self.check(&Token::RightParen) {
            loop {
                let parameter = match self.advance() {
                    Token::Identifier(name) => name,
                    _ => {
                        return Err(ParseError {
                            message: "Expected parameter name".into(),
                        })
                    }
                };

                // Optional type annotation: `name: string`
                if self.check(&Token::Colon) {
                    self.advance();
                    match self.advance() {
                        Token::Identifier(_) => {}
                        _ => {
                            return Err(ParseError {
                                message: "Expected type name after `:`".into(),
                            })
                        }
                    }
                }

                params.push(parameter);

                if !self.check(&Token::Comma) {
                    break;
                }
                self.advance();
            }
        }

        self.consume(
            &Token::RightParen,
            "Expected `)` after parameters",
        )?;

        // Optional return type: `-> int`
        if self.check(&Token::Arrow) {
            self.advance();
            match self.advance() {
                Token::Identifier(_) => {}
                _ => {
                    return Err(ParseError {
                        message: "Expected return type after `->`".into(),
                    })
                }
            }
        }

        let body = self.block()?;

        Ok(Stmt::Function {
            name,
            params,
            body,
        })
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let condition = self.expression()?;
        let then_branch = self.block()?;

        let else_branch = if self.check(&Token::Else) {
            self.advance();

            // Support `else if` without adding a new AST node.
            if self.check(&Token::If) {
                vec![self.if_stmt()?]
            } else {
                self.block()?
            }
        } else {
            Vec::new()
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.advance();
        let condition = self.expression()?;
        let body = self.block()?;

        Ok(Stmt::While { condition, body })
    }

    fn for_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.advance();

        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => {
                return Err(ParseError {
                    message: "Expected loop variable after `for`".into(),
                })
            }
        };

        self.consume(&Token::In, "Expected `in` after loop variable")?;
        let iterable = self.expression()?;
        let body = self.block()?;

        Ok(Stmt::For {
            name,
            iterable,
            body,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.consume(&Token::LeftBrace, "Expected `{`")?;

        let mut statements = Vec::new();
        while !self.check(&Token::RightBrace) && !self.check(&Token::EOF) {
            statements.push(self.statement()?);
        }

        self.consume(&Token::RightBrace, "Expected `}`")?;
        Ok(statements)
    }

    fn terminator(&mut self) -> Result<(), ParseError> {
        self.consume(
            &Token::Terminator,
            "Expected `*` or `!` at the end of the statement",
        )
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.or()
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.check(&Token::Or) {
            self.advance();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: "or".into(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.check(&Token::And) {
            self.advance();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: "and".into(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        loop {
            let operator = match self.peek() {
                Token::EqualEqual => "==",
                Token::NotEqual => "!=",
                _ => break,
            };

            self.advance();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator.into(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.range()?;

        loop {
            let operator = match self.peek() {
                Token::Greater => ">",
                Token::GreaterEqual => ">=",
                Token::Less => "<",
                Token::LessEqual => "<=",
                _ => break,
            };

            self.advance();
            let right = self.range()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator.into(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn range(&mut self) -> Result<Expr, ParseError> {
        let left = self.term()?;

        if self.check(&Token::Range) {
            self.advance();
            let right = self.term()?;
            return Ok(Expr::Range {
                start: Box::new(left),
                end: Box::new(right),
            });
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        loop {
            let operator = match self.peek() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };

            self.advance();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator.into(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        loop {
            let operator = match self.peek() {
                Token::Star => "*",
                Token::Slash => "/",
                Token::Percent => "%",
                _ => break,
            };

            self.advance();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator.into(),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Token::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    op: "-".into(),
                    expr: Box::new(self.unary()?),
                })
            }
            Token::Not | Token::Bang => {
                self.advance();
                Ok(Expr::Unary {
                    op: "not".into(),
                    expr: Box::new(self.unary()?),
                })
            }
            _ => self.call(),
        }
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut name = match self.primary()? {
            Expr::Identifier(name) => name,
            expression => return Ok(expression),
        };

        loop {
            // Allow dotted built-ins such as `web.listen()`.
            if self.check(&Token::Dot) {
                self.advance();
                let part = match self.advance() {
                    Token::Identifier(part) => part,
                    _ => {
                        return Err(ParseError {
                            message: "Expected identifier after `.`".into(),
                        })
                    }
                };
                name.push('.');
                name.push_str(&part);
                continue;
            }

            if self.check(&Token::LeftParen) {
                self.advance();
                let mut args = Vec::new();

                if !self.check(&Token::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }

                self.consume(
                    &Token::RightParen,
                    "Expected `)` after arguments",
                )?;

                return Ok(Expr::Call { name, args });
            }

            break;
        }

        Ok(Expr::Identifier(name))
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Token::Number(number) => Ok(Expr::Number(number)),
            Token::String(string) => Ok(Expr::String(string)),
            Token::True => Ok(Expr::Bool(true)),
            Token::False => Ok(Expr::Bool(false)),
            Token::Identifier(name) => Ok(Expr::Identifier(name)),
            Token::LeftParen => {
                let expression = self.expression()?;
                self.consume(&Token::RightParen, "Expected `)`")?;
                Ok(expression)
            }
            _ => Err(ParseError {
                message: "Expected expression".into(),
            }),
        }
    }
}
