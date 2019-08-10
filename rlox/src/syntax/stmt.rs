use syntax::Expr;

pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>)
}
