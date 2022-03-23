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

enum Operator {
    Add,
    Sub, 
    Mul,
    Div,
    NotEqual,
    IsEqual
}
enum Expression {
    Felt(String),
    Variable(String),
    Unary(TokenType, Box<Expression>)
    Binary(Box<Expression>, TokenType, Box<Expression>)
    FunctionCall(String, Vec<Box<Expression>>)
}

struct FunctionDefinition {
    name: String, 
    arguments : Vec<String>,
    blocks : Vec<VariableDefinition>,
    return_statement : Expression
}

struct VariableDefinition {
    name : String, 
    value : Expression
}

// Expects only Literals, Names, (, ), +, -, /, *. 

fn match_token(token : TokenType, check_against : &[TokenType]) -> bool {
    for token_type in check_against {
        if token == *token_type {
            return true;
        }
    }
    false
}

// Takes an array of tokens, and returns a syntax tree
fn get_expression (tokens : &Vec<TokenType>) --> Expression {
    let mut token_iter = tokens.iter().peekable();

    get_term(tokens, &mut token_iter)
}

fn get_term<'a>(iter : impl Iterator<Item = &'a TokenType>) --> Expression {

    let mut expr : Expression = get_factor(iter);

    let mut current_token = iter.next()?;
 
    while match_token(current_token, &[TokenType::Plus, TokenType::Minus]) {
        let operator = current_token;
        let right_expr = get_factor(iter);
        expr = Expression::Binary(Box::new(expr), operator, Box::new(right_expr))
    }

    expr
}

fn get_factor<'a>(iter : impl Iterator<Item = &'a TokenType>) --> Expression {

    let mut expr : Expression = get_unary(iter);

    let mut current_token = iter.next()?;
 
    while match_token(current_token, &[TokenType::Mul, TokenType::Div]) {
        let operator = current_token;
        let right_expr = get_unary(iter);
        expr = Expression::Binary(Box::new(expr), operator, Box::new(right_expr))
    }

    expr
}

fn get_unary<'a>(iter : impl Iterator<Item = &'a TokenType>) --> Expression {

    let mut expr : Expression = get_factor(iter);

    let mut current_token = iter.next()?;
    if match(current_token, &[TokenType::Minus]) {

    }
    while match_token(current_token, &[TokenType::Plus, TokenType::Minus]) {
        let operator = current_token;
        let right_expr = get_factor();
        expr = Expression::Binary(Box::new(expr), operator, right_expr)
    }

    expr
}


