//mod crate::scanner;

//mod scanner;
use crate::scanner::TokenType;

/*

Function Definition Grammar
---------------------------
func name(a,b,c,...) (->(x,y,z...))?:
    variable definition
    variable definition,
    ...
end

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
    Grouping(Box<Expression>),
    FuncCall(FunctionCall)
}

struct FunctionDefinition {
    name: String, 
    arguments : Vec<String>,
    statements : Vec<Statement>,
    return_statement : Vec<Expression>
}

// Statements that can be found in a code block (inside a function, if statement, etc.)
enum Statement {
    VarDef(Box<VariableDefinition>),
    FuncCall(Box<FunctionCall>),
}


struct FunctionCall {
    func_name: String, 
    args : Vec<Expression>
}


struct VariableDefinition {
    name: String, 
    value: Expression
}

struct If {

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

    pub fn peek(&self) -> &TokenType {
        self.tokens.get(self.cursor).unwrap()
    }

    pub fn peek_n(&self, n : usize) -> &TokenType {
        self.tokens.get(self.cursor + n).unwrap()
    }

    pub fn next(&mut self) -> &TokenType {
        self.cursor += 1;
        self.tokens.get(self.cursor - 1).unwrap()
    }

    // Advances without returning the next element
    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn advance_n(&mut self, n : usize) {
        self.cursor += n;
    }



    pub fn next_assert(&mut self, check_against : TokenType) {
        assert_eq!(*self.next(), check_against);
    }
}

// Checks if the next token matches any of the token types in `check_against`
fn match_token(tokens_iter : &mut TokenIter, check_against : &[TokenType]) -> bool {
    let token = tokens_iter.peek(); 
    for token_type in check_against {
        if *token == *token_type {
            return true;
        }
    }
    false
}

// -------------- FUNCTION PARSING -------------- //
/*
 * 1. Expdect `func`
 * 2. Expect `name`
 * 3. Expect `(`(a_1 (`,` a_i)*)? ')'
 * 4. Expect (`->` a_1 (`,` a_i)*)?
 * 7. Expect ':'
 * 8. Expect (VariableDefinition | FuncCall)*
 * 9. Expect `return``(` (a_1 (`,` a_i)*) `)`
 * 10. Expect end
*/

pub fn get_func(tokens_iter : &mut TokenIter) -> FunctionDefinition {
    if *tokens_iter.next() == TokenType::Func {
        let func_name;
        let func_args;
        let func_statements;
        let func_return;

        func_name = (*tokens_iter.next()).clone();
        
        tokens_iter.next_assert(TokenType::LeftParen);

        func_args = Vec::new();

        if *tokens_iter.peek() != TokenType::RightParen {
            func_args.push((*tokens_iter.next()).clone()); // Pushing first argument
            
            while *tokens_iter.peek() == TokenType::Comma {
                tokens_iter.advance(); // Advancing for the comma
                func_args.push((*tokens_iter.next()).clone()); // Pushing first argument
            }
        }

        tokens_iter.next_assert(TokenType::RightParen);

        // So far we have: func name (a,b,c,...)

        // Advancing until`:` is reached, indicating the end of the function signature.
        // Javascript doesn't specify return args in function signatures so we can ignore those
        while *tokens_iter.next() != TokenType::Colon {

        }

        tokens_iter.next_assert(TokenType::NewLine);

        // While we don't get a return statement, keep getting new statements
        func_statements = Vec::new();

        while *tokens_iter.peek() != TokenType::Return {
            func_statements.push(get_statement(tokens_iter));
        }


        func_return = Vec::new();

        tokens_iter.advance(); // Advancing for the `return` token
        tokens_iter.next_assert(TokenType::LeftParen);

        while *tokens_iter.peek() != TokenType:: NewLine {
            tokens_iter.advance_n(2); // Advancing for the name of the return variable, which we don't need in JS, and for the `=`

            func_return.push(get_term(tokens_iter));
        }

        tokens_iter.advance(); //Advancing for NewLine
        tokens_iter.next_assert(TokenType::End);

        return FunctionDefinition{
            name: func_name, 
            arguments: func_args, 
            statements: func_statements, 
            return_statement: func_return
        }

    } else {
        panic!("Expected a function definition");
    }
}

// -------------- STATEMENT PARSING -------------- //
fn get_statement(tokens_iter : &mut TokenIter) -> Statement {
    match *tokens_iter.next() {
        TokenType::Let => {
            match *tokens_iter.next() {
                TokenType::LeftParen => {
                    let var_name = (*tokens_iter.next().unwrap()).clone();
                    tokens_iter.next_assert(TokenType::RightParen); // Next token should be `)`. let (var_name)...
                    tokens_iter.next_assert(TokenType::Equals); // Next token should be `=`. let (var_name) = ...

                    let func_name = (*tokens_iter.next().unwrap()).clone();
                    tokens_iter.next_assert(TokenType::LeftParen); // Next token should be `(`. func_name(...
                    
                    let args = Vec::new();
                    while *tokens_iter.peek() != TokenType::NewLine {
                        args.push(get_term(tokens_iter));
                    }

                    tokens_iter.advance(); //Advancing for the NewLine token

                    Statement(VariableDefinition{
                        name: var_name, 
                        expression: Expression::FuncCall(
                            FunctionCall{
                                func_name: func_name, 
                                args: args
                            }
                        )
                    })

                },
                TokenType::Name(name_str) => {
                    let var_name = (*name_str).clone();

                    tokens_iter.next_assert(TokenType::Equals); // Next token should be `=`. let var_name = ...

                    let expr = get_term(tokens_iter);

                    // No need to advance for newline here as it's already skipped over by `get_term`
                    VariableDefinition{
                        name: var_name, 
                        expression: expr
                    }
                },
                _ => unreachable!()
            }

        },
        TokenType::Tempvar | TokenType::Local => {
            let var_name = (*tokens_iter.next().unwrap()).clone();
            
            tokens_iter.next_assert(TokenType::Equals); // Next token should be `=`. (tempvar|local) var_name = ...

            let expr = get_term(tokens_iter);

            VariableDefinition{
                name: var_name,
                expression: expr
            }
        },
        _ => unreachable!()
        // TODO: Add support for if statements
    }
}

// -------------- EXPRESSION PARSING -------------- //
/*
fn get_expression(tokens : Vec<TokenType>) -> Expression {
    let mut tokens_iter = TokenIter::new(tokens);

    get_comparison(&mut tokens_iter)
}*/

// Gets a (==|!=) b where a, b are sub expressions of the form x (+|-) y (+|-) z (+|-) ...
fn get_comparison(tokens_iter : &mut TokenIter) -> Expression {
    let mut expr = get_term(tokens_iter);

    if match_token(tokens_iter, &[TokenType::DoubleEquals, TokenType::NotEqual]) {
        let operator = (*tokens_iter.next()).clone();

        let right_expr = get_term(tokens_iter);

        expr = Expression::Binary(Box::new(expr), operator, Box::new(right_expr));
    }

    expr
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
            tokens_iter.next_assert(TokenType::RightParen);
            Expression::Grouping(Box::new(inner_expr))
        }
        _ => unreachable!()
    }
}
