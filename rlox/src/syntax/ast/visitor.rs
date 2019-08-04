use syntax::ast::*;

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, e: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, e: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, e: &UnaryExpr) -> T;
}
