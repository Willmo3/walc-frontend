use crate::lexer::Lexeme;
use crate::lexer::Lexeme::{CloseParen, Minus, OpenParen, Plus, Slash, Star};
use crate::token::Token;
use crate::token::Token::{Add, Divide, Multiply, Subtract};

/// Given an ordered collection of lexemes
/// Build an abstract syntax tree
pub fn parse(lexemes: Vec<Lexeme>) -> Token {
    let mut parser = Parser { index: 0, lexemes };
    parser.parse()
}

// Contain relevant data for parsing
struct Parser {
    index: usize,
    lexemes: Vec<Lexeme>,
}

// Parse methods
impl Parser {
    fn parse(&mut self) -> Token {
        self.parse_multiply()
    }

    fn parse_multiply(&mut self) -> Token {
        let mut left = self.parse_add();

        while self.in_bounds() {
            let operator = self.current();
            left = match operator {
                Star => {
                    self.advance();
                    Multiply { left: Box::new(left), right: Box::new(self.parse_add()) }
                }
                Slash => {
                    self.advance();
                    Divide { left: Box::new(left), right: Box::new(self.parse_add()) }
                }
                _ => { return left }
            }
        }

        left
    }

    fn parse_add(&mut self) -> Token {
        let mut left = self.parse_atom();

        while self.in_bounds() {
            let operator = self.current();
            left = match operator {
                Plus => {
                    self.advance();
                    Add { left: Box::new(left), right: Box::new(self.parse_atom()) }
                }
                Minus => {
                    self.advance();
                    Subtract { left: Box::new(left), right: Box::new(self.parse_atom()) }
                }
                _ => { return left }
            };
        }

        left
    }

    fn parse_atom(&mut self) -> Token {
        match self.current() {
            OpenParen => {
                self.advance();
                let value = self.parse();
                if !self.has(CloseParen) {
                    panic!("Unterminated parentheses!")
                }
                self.advance();
                value
            }
            _ => {
                self.parse_number()
            }
        }
    }

    fn parse_number(&mut self) -> Token {
        match self.next() {
            Lexeme::Number { value } => {
                Token::Number { value }
            }
            _ => { panic!("Invalid lexeme!"); }
        }
    }
}

// Parser helpers
impl Parser {
    fn in_bounds(&self) -> bool {
        self.index < self.lexemes.len()
    }

    // Return whether the cursor has an element of the specified type
    fn has(&self, l: Lexeme) -> bool {
        self.in_bounds() && self.lexemes[self.index] == l
    }

    // Advance to the next character.
    // Return the character that was previously under the cursor
    fn next(&mut self) -> Lexeme {
        assert!(self.in_bounds());
        let ret_val = self.current();
        self.index += 1;
        ret_val
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    // Get the current lexeme
    fn current(&self) -> Lexeme {
        assert!(self.in_bounds());
        self.lexemes[self.index]
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex;
    use crate::parser::parse;
    use crate::token::Token::{Add, Divide, Multiply, Number};

    #[test]
    fn test_lex() {
        let input = "(3 + 5) * 3 / -2";
        let lexemes = lex(input);

        let three = Number { value: 3.0 };
        let five = Number { value: 5.0 };
        let plus = Add { left: Box::new(three), right: Box::new(five) };
        let three = Number { value: 3.0 };
        let times = Multiply { left: Box::new(plus), right: Box::new(three) };
        let neg_two = Number { value: -2.0 };
        let divide = Divide { left: Box::new(times), right: Box::new(neg_two) };

        assert_eq!(divide, parse(lexemes));
    }
}