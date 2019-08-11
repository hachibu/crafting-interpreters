use syntax::*;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>, Position),
    Grouping(Box<Expr>, Position),
    Unary(Token, Box<Expr>, Position),
    Literal(Literal, Position),
    Variable(String, Position)
}
