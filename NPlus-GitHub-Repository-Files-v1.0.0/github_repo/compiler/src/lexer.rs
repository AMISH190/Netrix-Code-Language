use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LexError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

fn star_is_multiplication(chars: &[char], i: usize) -> bool {
    let mut j = i + 1;

    while j < chars.len() {
        match chars[j] {
            ' ' | '\t' | '\r' => j += 1,
            '\n' => return false,
            '}' => return false,
            _ => return true,
        }
    }

    false
}

pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut column = 1usize;
    let mut out = Vec::new();

    while i < chars.len() {
        let c = chars[i];

        match c {
            ' ' | '\t' | '\r' => {
                i += 1;
                column += 1;
            }
            '\n' => {
                i += 1;
                line += 1;
                column = 1;
            }
            '/' if i + 1 < chars.len() && chars[i + 1] == '/' => {
                i += 2;
                column += 2;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                    column += 1;
                }
            }
            '/' if i + 1 < chars.len() && chars[i + 1] == '*' => {
                let start_line = line;
                let start_column = column;
                i += 2;
                column += 2;

                loop {
                    if i + 1 >= chars.len() {
                        return Err(LexError {
                            line: start_line,
                            column: start_column,
                            message: "Unterminated block comment".into(),
                        });
                    }
                    if chars[i] == '*' && chars[i + 1] == '/' {
                        i += 2;
                        column += 2;
                        break;
                    }
                    if chars[i] == '\n' {
                        i += 1;
                        line += 1;
                        column = 1;
                    } else {
                        i += 1;
                        column += 1;
                    }
                }
            }
            '"' => {
                let start_line = line;
                let start_column = column;
                i += 1;
                column += 1;
                let mut value = String::new();

                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        i += 1;
                        column += 1;
                        let escaped = match chars[i] {
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            '"' => '"',
                            '\\' => '\\',
                            other => other,
                        };
                        value.push(escaped);
                        i += 1;
                        column += 1;
                    } else {
                        value.push(chars[i]);
                        if chars[i] == '\n' {
                            i += 1;
                            line += 1;
                            column = 1;
                        } else {
                            i += 1;
                            column += 1;
                        }
                    }
                }

                if i >= chars.len() {
                    return Err(LexError {
                        line: start_line,
                        column: start_column,
                        message: "Unterminated string".into(),
                    });
                }

                i += 1;
                column += 1;
                out.push(Token::String(value));
            }
            '0'..='9' => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                    column += 1;
                }

                if i < chars.len()
                    && chars[i] == '.'
                    && i + 1 < chars.len()
                    && chars[i + 1].is_ascii_digit()
                {
                    i += 1;
                    column += 1;
                    while i < chars.len() && chars[i].is_ascii_digit() {
                        i += 1;
                        column += 1;
                    }
                }

                let text: String = chars[start..i].iter().collect();
                let number = text.parse::<f64>().map_err(|_| LexError {
                    line,
                    column,
                    message: format!("Invalid number `{text}`"),
                })?;
                out.push(Token::Number(number));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                while i < chars.len()
                    && (chars[i].is_ascii_alphanumeric() || chars[i] == '_')
                {
                    i += 1;
                    column += 1;
                }

                let text: String = chars[start..i].iter().collect();
                let token = match text.as_str() {
                    "let" => Token::Let,
                    "var" => Token::Var,
                    "const" => Token::Const,
                    "fn" => Token::Fn,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "in" => Token::In,
                    "true" => Token::True,
                    "false" => Token::False,
                    "and" => Token::And,
                    "or" => Token::Or,
                    "not" => Token::Not,
                    _ => Token::Identifier(text),
                };
                out.push(token);
            }
            '*' => {
                if star_is_multiplication(&chars, i) {
                    out.push(Token::Star);
                } else {
                    out.push(Token::Terminator);
                }
                i += 1;
                column += 1;
            }
            '!' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                out.push(Token::NotEqual);
                i += 2;
                column += 2;
            }
            '!' => {
                out.push(Token::Terminator);
                i += 1;
                column += 1;
            }
            '+' => {
                out.push(Token::Plus);
                i += 1;
                column += 1;
            }
            '-' if i + 1 < chars.len() && chars[i + 1] == '>' => {
                out.push(Token::Arrow);
                i += 2;
                column += 2;
            }
            '-' => {
                out.push(Token::Minus);
                i += 1;
                column += 1;
            }
            '/' => {
                out.push(Token::Slash);
                i += 1;
                column += 1;
            }
            '%' => {
                out.push(Token::Percent);
                i += 1;
                column += 1;
            }
            '=' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                out.push(Token::EqualEqual);
                i += 2;
                column += 2;
            }
            '=' => {
                out.push(Token::Equal);
                i += 1;
                column += 1;
            }
            '>' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                out.push(Token::GreaterEqual);
                i += 2;
                column += 2;
            }
            '>' => {
                out.push(Token::Greater);
                i += 1;
                column += 1;
            }
            '<' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                out.push(Token::LessEqual);
                i += 2;
                column += 2;
            }
            '<' => {
                out.push(Token::Less);
                i += 1;
                column += 1;
            }
            '.' if i + 1 < chars.len() && chars[i + 1] == '.' => {
                out.push(Token::Range);
                i += 2;
                column += 2;
            }
            '(' => {
                out.push(Token::LeftParen);
                i += 1;
                column += 1;
            }
            ')' => {
                out.push(Token::RightParen);
                i += 1;
                column += 1;
            }
            '{' => {
                out.push(Token::LeftBrace);
                i += 1;
                column += 1;
            }
            '}' => {
                out.push(Token::RightBrace);
                i += 1;
                column += 1;
            }
            ',' => {
                out.push(Token::Comma);
                i += 1;
                column += 1;
            }
            ':' => {
                out.push(Token::Colon);
                i += 1;
                column += 1;
            }
            '.' => {
                out.push(Token::Dot);
                i += 1;
                column += 1;
            }
            _ => {
                return Err(LexError {
                    line,
                    column,
                    message: format!("Unexpected character `{c}`"),
                });
            }
        }
    }

    out.push(Token::EOF);
    Ok(out)
}
