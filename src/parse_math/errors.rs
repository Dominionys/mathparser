use std::fmt;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnableToParse(String),
    ParenthesisNotBalanced,
    InvalidOperator(String),
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            ParseError::ParenthesisNotBalanced => write!(f, "Balance parenthesis error"),
            ParseError::InvalidOperator(e) => write!(f, "Invalid operator: {}", e),
            ParseError::InvalidNumber(e) => write!(f, "Invalid number: {}", e),
        }
    }
}
