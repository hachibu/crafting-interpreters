use syntax::*;

#[derive(Debug)]
pub enum Stmt {
    Expr(Box<Expr>, Position),
    Print(Box<Expr>, Position),
    Var(String, Option<Box<Expr>>, Position)
}
