use std::fmt;
use yansi::Color;

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
        let pretty_error_message = format!(
            "{error}: {error_message}",
            error = Color::Red.paint("RuntimeError"),
            error_message = self.message
        );
        write!(f, "{}", pretty_error_message)
    }
}
