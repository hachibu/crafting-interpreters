use std::fmt;

pub struct RuntimeError {
    pub message: String
}

impl RuntimeError {
    pub fn new(message: String) -> RuntimeError {
        RuntimeError { message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
