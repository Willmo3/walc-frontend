use std::io::stdin;
use crate::lexer::lex;
use crate::parser::parse;

mod lexer;
mod parser;

// Very simple calculator driver.
// Basically no error checking
fn main() {
    // Repeatedly read lines from stdin
    // Repeatedly write answers to stdout
    let mut buffer = String::new();
    while stdin().read_line(&mut buffer).unwrap() > 0 {
        println!("{}", parse(lex(&buffer)).unwrap().evaluate());
        buffer = String::new();
    }
}