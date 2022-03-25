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

// Token
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

    pub fn peek(&self) -> &TokenType {
        self.tokens.get(self.cursor).unwrap()
    }

    pub fn next(&mut self) -> &TokenType {
        self.cursor += 1;
        self.tokens.get(self.cursor - 1).unwrap()
    }

    // Advances without returning the next element
    pub fn advance(&mut self) {
        self.cursor += 1;
    }
}

// Checks if the next token matches any of the token types in `check_against`
fn match_token(token_iter : &mut TokenIter, check_against : &[TokenType]) -> bool {
    let token = token_iter.peek(); 
    for token_type in check_against {
        if *token == *token_type {
            return true;
        }
    }
    false
}

pub fn get_expression(tokens : Vec<TokenType>) -> Expression {
    let mut tokens_iter = TokenIter::new(tokens);

    get_term(&mut tokens_iter)
}

// Gets a (+|-) b (+|-) c (+|-) ... where a, b, c, ... are sub expressions of the form x (*|/) y (*|/) z (*|/) ...
fn get_term(tokens_iter : &mut TokenIter) -> Expression {
    let mut expr = get_factor(tokens_iter);

    while match_token(tokens_iter, &[TokenType::Plus, TokenType::Minus]) {
        let operator = (*tokens_iter.next()).clone();

        let right_expr = get_factor(tokens_iter);

        expr = Expression::Binary(Box::new(expr), operator, Box::new(right_expr));
    }

    expr
}



// Gets x (*|/) y (*|/) z (*|/) ... where x, y, z, ... are sub expressions of the form (-)* u (0 or more "-" followed by a sub expression u)
fn get_factor(tokens_iter : &mut TokenIter) -> Expression {
    let  mut expr = get_unary(tokens_iter);

    while match_token(tokens_iter, &[TokenType::Mul, TokenType::Div]) {
        let operator = (*tokens_iter.next()).clone();

        let right_expr = get_unary(tokens_iter);

        expr = Expression::Binary(Box::new(expr), operator, Box::new(right_expr));
    }

    expr
}

// gets (-)* u where u is either a literal, a variable name, or a grouping (an expression inside parentheses)
fn get_unary(tokens_iter : &mut TokenIter) -> Expression {

    let next_token = (*tokens_iter.peek()).clone();

    if next_token == TokenType::Minus {
        tokens_iter.advance();
        let right_expr = get_unary(tokens_iter);
        return Expression::Unary(next_token, Box::new(right_expr));
    }

    get_primary(tokens_iter)
}

fn get_primary(tokens_iter : &mut TokenIter) -> Expression {

    let next_token = tokens_iter.next();

    match next_token {
        TokenType::Literal(some_str) => Expression::Felt(some_str.to_string()),
        TokenType::Name(some_str) => Expression::Variable(some_str.to_string()),
        TokenType::LeftParen => {
            let inner_expr = get_term(tokens_iter);
            assert_eq!(*tokens_iter.next(), TokenType::RightParen);
            return Expression::Grouping(Box::new(inner_expr));
        }
        _ => unreachable!()
    }
}
