use syntax::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }

    pub fn print_stmt(&mut self, s: &Stmt) {
        println!("{}", self.visit_stmt(s));
    }

    pub fn print_expr(&mut self, e: &Expr) {
        println!("{}", self.visit_expr(e));
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_stmt(&mut self, s: &Stmt) -> String {
        match s {
            Stmt::Expr(expression) => self.visit_expr(expression),
            Stmt::Print(expression) => format!(
                "(print {})",
                self.visit_expr(expression)
            ),
            Stmt::PrintAst(expression) => format!(
                "(printAst {})",
                self.visit_expr(expression)
            )
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
                Literal::String(v) => format!("{:?}", v),
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
