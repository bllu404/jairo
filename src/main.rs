//use std::env;
use std::fs;

mod scanner;
mod ast;

use scanner::{TokenType, scan};


fn main() {
    let contents = fs::read_to_string("example-programs/1.cairo")
        .expect("Something went wrong reading the file");

    let scanned = scan(contents);
    
    println!("{:?}", scanned);
}
