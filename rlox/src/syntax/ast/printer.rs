use syntax::ast::{Visitor, Stmt, Expr};

#[derive(Debug)]
pub struct Printer;

impl Printer {
    pub fn new() -> Printer {
        Printer {}
    }

    pub fn print(&mut self, e: &Expr) -> () {
        println!("{:}",  self.visit_expr(e))
    }
}

impl Visitor<String> for Printer {
    fn visit_stmt(&mut self, s: &Stmt) -> String{
        match s {
            Stmt::Expr(ref e) => self.visit_expr(e)
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary(a, t, b) => {
                format!(
                    "({:} {:} {:})",
                    t.to_string(),
                    self.visit_expr(&a),
                    self.visit_expr(&b)
                )
            },
            Expr::Unary(t, a) => {
                format!(
                    "({:} {:})",
                    t.to_string(),
                    self.visit_expr(&a)
                )
            },
            Expr::Grouping(a) => {
                format!("(group {:})", self.visit_expr(&a))
            },
            Expr::NumberLiteral(v) => {
                format!("{:}", v)
            },
            Expr::StringLiteral(v) => {
                format!("{:}", v)
            },
            Expr::BooleanLiteral(v) => {
                format!("{:}", v)
            },
            Expr::NilLiteral => {
                format!("nil")
            },
        }
    }
}
