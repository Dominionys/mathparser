use super::ast::Node;
use super::errors::ParseError;
use super::token::{OperationPrecedence, Token, Tokenizer};
use std::iter::Peekable;

pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(value: &'a str) -> Self {
        let tokenizer = Tokenizer::new(value).peekable();

        Parser { tokenizer }
    }

    pub fn evaluate(&mut self) -> Result<f64, ParseError> {
        let result = self.parse()?.eval();

        Ok(result)
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.ast(OperationPrecedence::Default)
    }
}

impl<'a> Parser<'a> {
    fn ast(&mut self, operation_precedence: OperationPrecedence) -> Result<Node, ParseError> {
        let mut left = self.number()?;

        loop {
            match self.tokenizer.peek() {
                Some(Token::EOF) => break,
                Some(operation) => {
                    if operation_precedence >= operation.operation_precedence() {
                        break;
                    }

                    left = self.operation(left)?;
                }
                None => {
                    return Err(ParseError::UnableToParse("Unknown char".into()));
                }
            }
        }
        Ok(left)
    }

    fn number(&mut self) -> Result<Node, ParseError> {
        let current_token = self
            .tokenizer
            .next()
            .ok_or(ParseError::UnableToParse("Number parse error".into()))?;

        let node = match current_token {
            Token::Plus => self.number()?,
            Token::Minus => Node::Negative(Box::new(self.number()?)),
            Token::Number(number) => Node::Element(number),
            Token::LeftParenthesis => {
                let ast = self.ast(OperationPrecedence::Default)?;

                if self.tokenizer.next() != Some(Token::RightParenthesis) {
                    return Err(ParseError::ParenthesisNotBalanced);
                }

                ast
            }
            token => {
                return Err(ParseError::InvalidNumber(format!("{:?}", token).into()));
            }
        };

        Ok(node)
    }

    fn operation(&mut self, left: Node) -> Result<Node, ParseError> {
        let current_token = self
            .tokenizer
            .next()
            .ok_or(ParseError::UnableToParse("Operator parse error".into()))?;

        let operation_precedence = current_token.operation_precedence();
        let node = match current_token {
            Token::Plus => {
                let right = self.ast(operation_precedence)?;
                Node::Sum(Box::new(left), Box::new(right))
            }
            Token::Minus => {
                let right = self.ast(operation_precedence)?;
                Node::Subtract(Box::new(left), Box::new(right))
            }
            Token::Asterisk => {
                let right = self.ast(operation_precedence)?;
                Node::Multiply(Box::new(left), Box::new(right))
            }
            Token::Slash => {
                let right = self.ast(operation_precedence)?;
                Node::Divide(Box::new(left), Box::new(right))
            }
            Token::Caret => {
                let right = self.ast(operation_precedence)?;
                Node::Power(Box::new(left), Box::new(right))
            }
            Token::LeftParenthesis => {
                let right = self.ast(OperationPrecedence::Default)?;
                if self.tokenizer.next() != Some(Token::RightParenthesis) {
                    return Err(ParseError::ParenthesisNotBalanced);
                }

                Node::Multiply(Box::new(left), Box::new(right))
            }
            token => {
                return Err(ParseError::InvalidOperator(format!("{:?}", token).into()));
            }
        };

        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_test() {
        let mut parser = Parser::new("-1");
        let ast = parser.parse();
        let expected = Node::Negative(Box::new(Node::Element(1.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn trim_plus() {
        let mut parser = Parser::new("+1");
        let ast = parser.parse();
        let expected = Node::Element(1.);
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn sum_two() {
        let mut parser = Parser::new("1+2");
        let ast = parser.parse();
        let expected = Node::Sum(Box::new(Node::Element(1.)), Box::new(Node::Element(2.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn sum_many() {
        let mut parser = Parser::new("10+20+30");
        let ast = parser.parse();
        let left = Node::Sum(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let expected = Node::Sum(Box::new(left), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn multiply_two() {
        let mut parser = Parser::new("1*2");
        let ast = parser.parse();
        let expected = Node::Multiply(Box::new(Node::Element(1.)), Box::new(Node::Element(2.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn multiply_many() {
        let mut parser = Parser::new("10*20*30");
        let ast = parser.parse();
        let left = Node::Multiply(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let expected = Node::Multiply(Box::new(left), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn divide_two() {
        let mut parser = Parser::new("1/2");
        let ast = parser.parse();
        let expected = Node::Divide(Box::new(Node::Element(1.)), Box::new(Node::Element(2.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn divide_many() {
        let mut parser = Parser::new("10/20/30");
        let ast = parser.parse();
        let left = Node::Divide(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let expected = Node::Divide(Box::new(left), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn subtract_two() {
        let mut parser = Parser::new("1-2");
        let ast = parser.parse();
        let expected = Node::Subtract(Box::new(Node::Element(1.)), Box::new(Node::Element(2.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn subtract_many() {
        let mut parser = Parser::new("10-20-30");
        let ast = parser.parse();
        let left = Node::Subtract(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let expected = Node::Subtract(Box::new(left), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn power_two() {
        let mut parser = Parser::new("1^2");
        let ast = parser.parse();
        let expected = Node::Power(Box::new(Node::Element(1.)), Box::new(Node::Element(2.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn pow_many() {
        let mut parser = Parser::new("10^20^30");
        let ast = parser.parse();
        let left = Node::Power(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let expected = Node::Power(Box::new(left), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn combine_pow() {
        let mut parser = Parser::new("3^2*2");
        let ast = parser.parse();
        let left = Node::Power(Box::new(Node::Element(3.)), Box::new(Node::Element(2.)));
        let expected = Node::Multiply(Box::new(left), Box::new(Node::Element(2.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn combine_1() {
        let mut parser = Parser::new("10+20*30");
        let ast = parser.parse();
        let right = Node::Multiply(Box::new(Node::Element(20.)), Box::new(Node::Element(30.)));
        let expected = Node::Sum(Box::new(Node::Element(10.)), Box::new(right));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn combine_2() {
        let mut parser = Parser::new("10*20+30");
        let ast = parser.parse();
        let left = Node::Multiply(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let expected = Node::Sum(Box::new(left), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn parenthesis() {
        let mut parser = Parser::new("(20+30)");
        let ast = parser.parse();
        let expected = Node::Sum(Box::new(Node::Element(20.)), Box::new(Node::Element(30.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn combine_parenthesis() {
        let mut parser = Parser::new("10*(20+30)");
        let ast = parser.parse();
        let right = Node::Sum(Box::new(Node::Element(20.)), Box::new(Node::Element(30.)));
        let expected = Node::Multiply(Box::new(Node::Element(10.)), Box::new(right));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn combine_parenthesis_multiply_1() {
        let mut parser = Parser::new("(10)(20)");
        let ast = parser.parse();
        let expected = Node::Multiply(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        assert_eq!(ast, Ok(expected))
    }

    #[test]
    fn combine_parenthesis_multiply_2() {
        let mut parser = Parser::new("(10+20)(30+40)");
        let ast = parser.parse();
        let left = Node::Sum(Box::new(Node::Element(10.)), Box::new(Node::Element(20.)));
        let right = Node::Sum(Box::new(Node::Element(30.)), Box::new(Node::Element(40.)));
        let expected = Node::Multiply(Box::new(left), Box::new(right));
        assert_eq!(ast, Ok(expected))
    }
}
