use syntax::*;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(String, Option<Box<Expr>>)
}
