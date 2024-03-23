// Basic tree-based calculator.
// Author: Willmo3

pub enum TokenType {
    Number, // Coerce all numbers to floats
    Plus,
    Minus,
    Star,
    Slash,
}

pub enum Token {
    Number { value: f64 },
    Add { left: Box<Token>, right: Box<Token> },
    Subtract { left: Box<Token>, right: Box<Token> },
    Multiply { left: Box<Token>, right: Box<Token> },
    Divide { left: Box<Token>, right: Box<Token> },
}

// Since this is just a frontend, don't do any evaluation!
// Simply export the abstract syntax tree to JSON
impl Token {

}