use std::fmt;

#[derive(Debug)]
pub enum LoxObject {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String)
}

impl fmt::Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxObject::Boolean(v) => write!(f, "{}", v),
            LoxObject::Number(v) => write!(f, "{}", v),
            LoxObject::Nil => write!(f, "nil"),
            LoxObject::String(v) => write!(f, "{}", v)
        }
    }
}
