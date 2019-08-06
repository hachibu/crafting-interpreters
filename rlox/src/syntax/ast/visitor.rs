use syntax::ast::*;

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}
