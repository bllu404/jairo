use regex::Regex;

#[derive(Debug)]
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

    // Key words
    Func, 
    Let, 
    Tempvar, 
    Local, 
    If, 
    Return, 
    End, 

    // Variables and Literals
    Literal(String),
    Variable(String)
}

pub fn scan(cairo_code : String) -> Vec<TokenType> {

    let mut tokens: Vec<TokenType> = Vec::new();

    let mut current_token = String::new();

    for (i,c) in cairo_code.chars().enumerate() {
        match c {
            ' ' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
            },
            '(' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::LeftParen);
            },
            ')' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::RightParen);
            },
            ':' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::Colon);
            },
            '+' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::Plus);
            },
            '-' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::Minus);
            },
            '*' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::Mul);
            },
            '/' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::Div);
            },
            '.' => {
                if current_token.len() > 0 {
                    tokens.push(match_token(&current_token));
                    current_token.clear()
                }
                tokens.push(TokenType::Dot);
            },
            _ => {
                current_token.push(c);
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
            let checkFelt = Regex::new(r"/^\d+$/").unwrap();
            if checkFelt.is_match(token.as_str()) {
                return TokenType::Literal(token.to_string());
            } else {
                return TokenType::Variable(token.to_string());
            }
        }
    }
}