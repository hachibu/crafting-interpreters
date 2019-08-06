use std::fmt;

pub struct SyntaxError {
    pub message: String
}

impl SyntaxError {
    pub fn new(message: String) -> SyntaxError {
        SyntaxError { message }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
