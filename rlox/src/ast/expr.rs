use syntax::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(String),
    NilLiteral,
    Unary(Token, Box<Expr>)
}

impl Expr {
    pub fn into_boxed(self) -> Box<Expr> {
        Box::new(self)
    }
}
