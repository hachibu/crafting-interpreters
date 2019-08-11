use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LoxErrorTy {
    Runtime,
    Syntax
}

impl fmt::Display for LoxErrorTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LoxErrorTy::Runtime => "RuntimeError",
            LoxErrorTy::Syntax => "SyntaxError"
        };
        write!(f, "{}", s)
    }
}
