//use std::env;
use std::fs;

mod scanner;
mod ast;

use scanner::{TokenType, scan};
use ast::ExprParserState;


fn main() {
    let contents = fs::read_to_string("example-programs/1.cairo")
        .expect("Something went wrong reading the file");

    //let scanned = scan(contents);
    //let parsed_expr = get_expression(&vec![TokenType::Literal(String::from("1")), TokenType::Plus, TokenType::Literal(String::from("2"))]);
    
    //println!("{:?}", parsed_expr);

    let mut state = ExprParserState::new(vec![TokenType::LeftParen, TokenType::RightParen, TokenType::LeftParen, TokenType::LeftParen]);
    println!("{}", state.cursor());


    println!("{:?}", state.next());
    println!("{:?}", state.next());
    println!("{}", state.cursor());
}
