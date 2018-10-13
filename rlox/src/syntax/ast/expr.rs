use syntax::lex::Token;

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
