//use std::env;
use std::fs;
mod scanner;

fn main() {
    let contents = fs::read_to_string("example-programs/1.cairo")
        .expect("Something went wrong reading the file");

    let scanned = scanner::scan(contents);

    println!("{:?}", scanned);
}
