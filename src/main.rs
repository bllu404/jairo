//use std::env;
use std::fs;

mod scanner;

fn main() {
    println!("Hello, world!");

    println!("In file {}", "test.txt");

    let contents = fs::read_to_string("example-programs/1.cairo")
        .expect("Something went wrong reading the file");

    let scanned = scanner::scan(contents);

    println!("{:?}", scanned);
}
