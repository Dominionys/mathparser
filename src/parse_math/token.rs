use std::iter::{Filter, Peekable};
use std::str::Chars;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum OperationPrecedence {
    Default,
    AddSub,
    MulDiv,
    Power,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    LeftParenthesis,
    RightParenthesis,
    EOF,
}

impl Token {
    pub fn operation_precedence(&self) -> OperationPrecedence {
        match self {
            Self::Plus | Self::Minus => OperationPrecedence::AddSub,
            Self::Asterisk | Self::Slash | Self::LeftParenthesis => OperationPrecedence::MulDiv,
            Self::Caret => OperationPrecedence::Power,
            _ => OperationPrecedence::Default,
        }
    }
}

pub struct Tokenizer<'a> {
    chars: Peekable<Filter<Chars<'a>, &'a dyn Fn(&char) -> bool>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        let chars = expression
            .chars()
            .filter((&|char: &char| !char.is_ascii_whitespace()) as &'a dyn Fn(&char) -> bool)
            .peekable();
        Tokenizer { chars }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.token()
    }
}

impl<'a> Tokenizer<'a> {
    fn token(&mut self) -> Option<Token> {
        let next_char = self.chars.next();

        let char = match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.chars.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.chars.next()?);
                    } else {
                        break;
                    }
                }

                Token::Number(number.parse::<f64>().unwrap())
            }
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('^') => Token::Caret,
            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,
            Some(_) => {
                return None;
            }
            None => Token::EOF,
        };
        Some(char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_number() {
        let mut tokenizer = Tokenizer::new("1").peekable();
        assert_eq!(tokenizer.peek(), Some(&Token::Number(1.)));
        assert_eq!(tokenizer.next(), Some(Token::Number(1.)));
        assert_eq!(tokenizer.peek(), None);
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_int_number() {
        let mut tokenizer = Tokenizer::new("1234567890").peekable();

        assert_eq!(tokenizer.peek(), Some(&Token::Number(1234567890.)));
        assert_eq!(tokenizer.next(), Some(Token::Number(1234567890.)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn parse_float_number() {
        let mut tokenizer = Tokenizer::new("1234567890.1234567890");

        assert_eq!(tokenizer.next(), Some(Token::Number(1234567890.123456789)));
        assert_eq!(tokenizer.next(), None);
    }
}
