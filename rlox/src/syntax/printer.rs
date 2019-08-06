use syntax::*;

pub struct Printer;

impl Printer {
    pub fn new() -> Printer {
        Printer {}
    }

    pub fn print(&mut self, s: &Stmt) {
        println!("{}", self.visit_stmt(s));
    }
}

impl Visitor<String> for Printer {
    fn visit_stmt(&mut self, s: &Stmt) -> String {
        match s {
            Stmt::Expr(expression) => self.visit_expr(expression)
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary(left, operator, right) => format!(
                "({} {} {})",
                operator.to_string(),
                self.visit_expr(left),
                self.visit_expr(right)
            ),
            Expr::Grouping(expression) => format!(
                "({})",
                self.visit_expr(expression)
            ),
            Expr::Literal(value) => match value {
                Literal::Number(v) => format!("{}", v),
                Literal::String(v) => format!("{}", v),
                Literal::Boolean(v) => format!("{}", v),
                Literal::Nil => String::from("nil")
            },
            Expr::Unary(operator, right) => format!(
                "({} {})",
                operator.to_string(),
                self.visit_expr(right)
            )
        }
    }
}
