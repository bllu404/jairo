//mod crate::scanner;

//mod scanner;
use crate::scanner::TokenType;

/*

Variable Definition Grammar
---------------------------
tempvar name = expression
let name = expression
let (name) = functionCall()
local name = expression
ddd
Expression Grammar
-------
expression -> term
term -> factor ( ( "-" | "+" ) factor)*
factor -> unary ( ( "/" | "*" ) unary)*
unary -> "-" ( unary | primary )
primary -> felt | name | ( expression )
*/

#[derive(Debug)]
pub enum Expression {
    Felt(String),
    Variable(String),
    Unary(TokenType, Box<Expression>),
    Binary(Box<Expression>, TokenType, Box<Expression>),
    Grouping(Box<Expression>)
}

struct FunctionCall {
    func_name: String, 
    args : Vec<Expression>
}

struct FunctionDefinition {
    name: String, 
    arguments : Vec<String>,
    blocks : Vec<VariableDefinition>,
    return_statement : Expression
}

struct VariableDefinition {
    name: String, 
    value: Expression
}

pub struct TokenIter {
    cursor: usize,
    tokens: Vec<TokenType>
}

impl TokenIter {
    pub fn new(tokens : Vec<TokenType>) -> Self {
        Self {
            cursor: 0,
            tokens: tokens
        }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&TokenType> {
        self.tokens.get(self.cursor)
    }

    pub fn next(&mut self) -> Option<&TokenType> {
        match self.tokens.get(self.cursor) {
            Some(token) => {
                self.cursor += 1;

                Some(token)
            }
            None => None
        }
    }

    // Advances without returning the next element
    pub fn advance(&mut self) {
        self.cursor += 1;
    }
}

fn match_token(token_iter : &mut TokenIter, check_against : &[TokenType]) -> Option<bool> {
    if let Some(token) = token_iter.peek() {
        for token_type in check_against {
            if *token == *token_type {
                return Some(true);
            }
        }
        return Some(false);
    }
    None
}

pub fn get_expression(tokens : Vec<TokenType>) -> Option<Expression> {
    let mut tokens_iter = TokenIter::new(tokens);

    get_term(&mut tokens_iter)
}

fn get_term(tokens_iter : &mut TokenIter) -> Option<Expression> {
    let expr = get_factor(tokens_iter);

    while let Some(true) = match_token(tokens_iter, &[TokenType::Plus, TokenType::Minus]) {
        if let Some(operator) = tokens_iter.next() {

            let operator = (*operator).clone();

            if let Some(right_expr) = get_factor(tokens_iter) {
                if let Some(left_expr) = expr {
                    return Some(Expression::Binary(Box::new(left_expr), operator, Box::new(right_expr)));
                }
            }
        }
    }
    expr
}

fn get_factor(tokens_iter : &mut TokenIter) -> Option<Expression> {
    let  expr = get_unary(tokens_iter);

    while let Some(true) = match_token(tokens_iter, &[TokenType::Mul, TokenType::Div]) {
        if let Some(operator) = tokens_iter.next() {
            
            let operator = (*operator).clone();
            if let Some(right_expr) = get_unary(tokens_iter) {
                if let Some(left_expr) = expr {
                    return Some(Expression::Binary(Box::new(left_expr), operator, Box::new(right_expr)));
                }
            }
        }
    }
    expr
}

fn get_unary(tokens_iter : &mut TokenIter) -> Option<Expression> {
    if let Some(some_token) = tokens_iter.peek() {

        let some_token = (*some_token).clone();

        if some_token == TokenType::Minus {
            tokens_iter.advance();
            
            if let Some(right_expr) = get_unary(tokens_iter) {
                return Some(Expression::Unary(some_token, Box::new(right_expr)));
            }
        }
    }

    get_primary(tokens_iter)
}

fn get_primary(tokens_iter : &mut TokenIter) -> Option<Expression> {
    if let Some(token) = tokens_iter.next() {
        return match token {
            TokenType::Literal(some_str) => Some(
                Expression::Felt(some_str.to_string())
            ),
            TokenType::Name(some_str) => Some(
                Expression::Variable(some_str.to_string())
            ),
            TokenType::LeftParen => {
                if let Some(inner_expr) = get_term(tokens_iter) {
                    tokens_iter.advance(); // Advancing the cursor for the closing ')'.
                    return Some(
                        Expression::Grouping(
                            Box::new(inner_expr)
                        )
                    );
                }

                None
            }
            _ => None
        };
    }
    None
}
