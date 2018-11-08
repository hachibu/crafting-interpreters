use syntax::ast::{Visitor, Stmt, Expr};

#[derive(Debug)]
pub struct Printer;

impl Visitor<()> for Printer {
    fn visit_stmt(&mut self, s: &Stmt) {
        match *s {
            Stmt::Expr(ref e) => self.visit_expr(e)
        }
    }

    fn visit_expr(&mut self, e: &Expr) {
        match *e {
            Expr::Binary(_, _, _) => {},
            Expr::Unary(_, _) => {},
            Expr::Grouping(_) => {},
            Expr::NumberLiteral(v) => {
                println!("{:?}", v)
            },
            Expr::StringLiteral(_) => {},
            Expr::BooleanLiteral(_) => {},
            Expr::NilLiteral => {},
        }
    }
}
