// Basic tree-based calculator.
// Author: Willmo3

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lexeme {
    Number { value: f64 }, // Coerce all numbers to floats
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Slash,
    // Special token that all files are terminated by
    EOF,
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn export(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    pub fn import(data: &str) -> Self {
        serde_json::from_str(data).unwrap()
    }
}