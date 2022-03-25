//use std::env;
use std::fs;

mod scanner;
mod ast;

use scanner::{TokenType, scan};
use ast::get_expression;


fn main() {
    let contents = fs::read_to_string("example-programs/1.cairo")
        .expect("Something went wrong reading the file");

    //let scanned = scan(contents);
    let parsed_expr = get_expression(vec![
        TokenType::LeftParen, 
        TokenType::Literal(String::from("1")), 
        TokenType::Plus, 
        TokenType::Literal(String::from("2")), 
        TokenType::Plus, 
        TokenType::Literal(String::from("3")), 
        TokenType::Plus, 
        TokenType::Literal(String::from("4")), 
        TokenType::Plus, 
        TokenType::Literal(String::from("5")), 
        TokenType::RightParen, 
        TokenType::Mul,
        TokenType::Literal(String::from("6")),
        TokenType::EoF
    ]);
    
    println!("{:?}", parsed_expr);

}
