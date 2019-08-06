use syntax::Expr;

pub enum Stmt {
    Expr(Box<Expr>)
}
