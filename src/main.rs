//use std::env;
use std::fs;

mod scanner;
mod ast;

use scanner::{scan};
//use ast::get_expression;


fn main() {
    let contents = fs::read_to_string("example-programs/1.cairo").expect("Something went wrong reading the file");
    //let scanned = scan("1+2*(3+4) != 4+5+6".to_string());
    //let parsed_expr = get_expression(scanned);
    
    println!("{:?}", contents);
}
