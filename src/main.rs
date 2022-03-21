use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");

    println!("In file {}", "test.txt");

    let contents = fs::read_to_string("example-programs/1.cairo")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
