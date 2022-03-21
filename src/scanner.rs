
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

    // Key words
    Func, 
    Let, 
    Tempvar, 
    Local, 
    If, 
    Return, 
    End, 

    // Literal
    Literal(String)
}


pub fn scan(cairo_code : String) -> Vec<TokenType> {

    let mut tokens: Vec<TokenType> = Vec::new();

    for c in cairo_code.chars() {
        tokens.push(TokenType::LeftParen);
    }

    tokens
}