//mod crate::scanner;

//mod scanner;
use crate::scanner::TokenType;
use std::iter::Peekable;
use std::iter::Iterator;

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
enum Expression {
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

pub struct ExprParserState {
    cursor: usize,
    tokens: Vec<TokenType>
}

impl ExprParserState {
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
}
/*
fn match_token(token : TokenType, check_against : &[TokenType]) -> bool {
    for token_type in check_against {
        if token == *token_type {
            return true;
        }
    }
    false
}

pub fn get_expression(tokens : &Vec<TokenType>) -> Option<Expression> {
    let mut state = ExprParserState::new(tokens);

    get_term(&mut state)
}

fn get_term(tokens : &mut state) -> Option<Expression> {

}

// Takes an array of tokens, and returns a syntax tree
pub fn get_expression (tokens : &Vec<TokenType>) -> Expression {
    let mut token_iter = tokens.iter().peekable();

    get_term(&mut token_iter)
}

fn get_term<'a, I>(iter: &mut Peekable<I>) -> Expression
where
    I : Iterator<Item = &'a TokenType>,
{
    let mut expr : Expression = get_factor(iter);

    if let Some(tok) = iter.peek() {
        let current_token = tok;

        while match_token(**current_token, &[TokenType::Plus, TokenType::Minus]) {
            iter.next();
            let right_expr = get_factor(iter);
            expr = Expression::Binary(Box::new(expr), **current_token, Box::new(right_expr))
        }
    }

    expr
}

fn get_factor<'a, I>(iter: &mut Peekable<I>) -> Expression
where
    I : Iterator<Item = &'a TokenType>,
{

    let mut expr : Expression = get_unary(iter);

    if let Some(tok) = iter.peek() {
        let current_token = tok;

        while match_token(**current_token, &[TokenType::Mul, TokenType::Div]) {
            iter.next();
            let right_expr = get_unary(iter);
            expr = Expression::Binary(Box::new(expr), **current_token, Box::new(right_expr))
        }
    } 

    expr
}

fn get_unary<'a, I>(iter: &mut Peekable<I>) -> Expression
where
    I : Iterator<Item = &'a TokenType>,
{
    if let Some(tok) = iter.peek() {
        let current_token = tok;

        if match_token(**current_token, &[TokenType::Minus]) {
            iter.next();
            let right = get_unary(iter);
            return Expression::Unary(**current_token, Box::new(right));
        }
    }

    get_primary(iter)
}

fn get_primary<'a, I>(iter: &mut Peekable<I>) -> Expression
where
    I : Iterator<Item = &'a TokenType>,
{
    if let Some(tok) = iter.next() {
        let current_token = tok;

        if let TokenType::Literal(lit) = *current_token {
            return Expression::Felt(lit);
        }
    
        if let TokenType::Name(name) = *current_token {
            return Expression::Variable(name);
        }
    
        if *current_token == TokenType::LeftParen {
            let expr = get_term(iter);
            iter.next(); //Advancing iter since a ')' is expected to come after the expression
            return Expression::Grouping(Box::new(expr));
        }
    }

    panic!("Invalid expression");
}

*/
