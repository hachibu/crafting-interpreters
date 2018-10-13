use syntax::ast::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr)
}
