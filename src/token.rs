// Basic tree-based calculator.
// Author: Willmo3

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Token {
    Number { value: f64 },
    Add { left: Box<Token>, right: Box<Token> },
    Subtract { left: Box<Token>, right: Box<Token> },
    Multiply { left: Box<Token>, right: Box<Token> },
    Divide { left: Box<Token>, right: Box<Token> },
}

impl Token {
    /// Evaluate the AST rooted at self.
    /// Return f64 result of computation
    pub fn evaluate(&self) -> f64 {
        match self {
            Token::Number { value } => { *value }
            Token::Add { left, right } => {
                left.evaluate() + right.evaluate()
            }
            Token::Subtract { left, right } => {
                left.evaluate() - right.evaluate()
            }
            Token::Multiply { left, right } => {
                left.evaluate() * right.evaluate()
            }
            Token::Divide { left, right } => {
                let (left, right) = (left.evaluate(), right.evaluate());
                if right == 0.0 {
                    panic!("Divide by zero!");
                }
                left / right
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::parser::parse;

    // For ease of use, using lex and parse.
    #[test]
    fn test_eval() {
        let input = "(3 - 3) *2 - 1+1 / 2";
        let expected = -0.5;
        let ast = parse(lex(input)).unwrap();
        assert_eq!(expected, ast.evaluate());
    }

    // one of the few special cases in my limited language
    #[test]
    #[should_panic]
    fn test_div_zero() {
        parse(lex("3/0")).unwrap().evaluate();
    }
}
