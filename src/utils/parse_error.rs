use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self {
            self::ParseError::UnableToParse(e) => e.clone(),
            self::ParseError::InvalidOperator(e) => e.clone(),
        };
        write!(f, "Error in evaluating {}", message)
    }
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: std::boxed::Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Unable to parse".into())
    }
}
