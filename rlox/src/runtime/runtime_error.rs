use std::fmt;

#[derive(Debug, PartialEq)]
pub struct RuntimeError<'a> {
    pub message: &'a str,
}

impl<'a> RuntimeError<'a> {
    pub fn new(message: &'a str) -> RuntimeError {
        RuntimeError { message }
    }
}

impl<'a> fmt::Display for RuntimeError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
