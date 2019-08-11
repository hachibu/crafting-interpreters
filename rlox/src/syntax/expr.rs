use syntax::*;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>, SourceMap),
    Grouping(Box<Expr>, SourceMap),
    Unary(Token, Box<Expr>, SourceMap),
    Literal(Literal, SourceMap),
    Variable(String, SourceMap)
}
