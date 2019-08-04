use syntax::ast::*;

#[derive(Debug)]
pub struct Printer;

impl Printer {
    pub fn new() -> Printer {
        Printer {}
    }

    pub fn print(&mut self, e: &Expr) {
        print!("{:}", e.accept(self))
    }
}

impl Visitor<String> for Printer {
    fn visit_binary_expr(&mut self, e: &BinaryExpr) -> String {
        format!(
            "({:} {:} {:})",
            e.operator.to_string(),
            e.left.accept(self),
            e.right.accept(self)
        )
    }

    fn visit_grouping_expr(&mut self, e: &GroupingExpr) -> String {
        format!("(group {:})", e.expression.accept(self))
    }

    fn visit_literal_expr(&mut self, e: &LiteralExpr) -> String{
        match &e.value {
            Literal::Number(v) => format!("{:}", v),
            Literal::String(v) => format!("{:}", v),
            Literal::Boolean(v) => format!("{:}", v),
            Literal::Nil => String::from("nil"),
        }
    }

    fn visit_unary_expr(&mut self, e: &UnaryExpr) -> String{
        format!(
            "({:} {:})",
            e.operator.to_string(),
            e.right.accept(self)
        )
    }
}
