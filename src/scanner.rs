use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen, 
    Colon,
    Equals, 
    Plus, 
    Minus,
    Mul, 
    Div, 
    Dot,
    Comma,

    //Multi-symbol tokens
    ReturnSpec,
    DoubleEquals,
    NotEqual,

    // Key words
    Func, 
    Let, 
    Tempvar, 
    Local, 
    If, 
    Return, 
    End, 
    NewLine,

    // Variables and Literals
    Literal(String),
    Name(String)
}

pub fn scan(cairo_code : String) -> Vec<TokenType> {

    let mut tokens: Vec<TokenType> = Vec::new();

    let mut current_token = String::new();

    let mut code_iter = cairo_code.chars().peekable();

    for i in 0..cairo_code.len() {
        let current_char = code_iter.next();
        if let Some(c) = current_char {
            match c {
                ' ' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                },
                '(' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::LeftParen);
                },
                ')' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::RightParen);
                },
                ':' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::Colon);
                },
                ',' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::Comma);
                },
                '+' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::Plus);
                },
                '-' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }

                    if code_iter.peek() == Some(&'>') {
                        tokens.push(TokenType::ReturnSpec);
                        code_iter.next();
                    } else {
                        tokens.push(TokenType::Minus);
                    }
                },
                '*' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::Mul);
                },
                '/' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::Div);
                },
                '.' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::Dot);
                },
                '=' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }

                    if code_iter.peek() == Some(&'=') {
                        tokens.push(TokenType::DoubleEquals);
                        code_iter.next();
                    } else {
                        tokens.push(TokenType::Equals);
                    }
                },
                '!' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }

                    if code_iter.peek() == Some(&'=') {
                        tokens.push(TokenType::NotEqual);
                        code_iter.next();
                    }
                },
                '\n' => {
                    if current_token.len() > 0 {
                        tokens.push(match_token(&current_token));
                        current_token.clear();
                    }
                    tokens.push(TokenType::NewLine);
                },
                _ => {
                    current_token.push(c);
                }
            }
        } else {
            if current_token.len() > 0 {
                tokens.push(match_token(&current_token))
            }
        }
    }

    tokens
}

// Identities whether a token is a key word, literal, or user-defined name (e.g., variable name, function name) and 
// returns the appropriate type
fn match_token(token : &String) -> TokenType {
    match token.as_str() {
        "func" => TokenType::Func,
        "let" => TokenType::Let,
        "tempvar" => TokenType::Tempvar, 
        "local" => TokenType::Local, 
        "return" => TokenType::Return,
        "end" => TokenType::End,
        "if" => TokenType::If,
        _ => {
            let check_felt = Regex::new(r"^\d+$").unwrap();
            if check_felt.is_match(token.as_str()) {
                return TokenType::Literal(token.to_string());
            } else {
                return TokenType::Name(token.to_string());
            }
        }
    }
}