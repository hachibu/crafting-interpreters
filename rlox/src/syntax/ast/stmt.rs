use syntax::ast::Expr;

pub enum Stmt {
    Expr(Box<Expr>)
}
