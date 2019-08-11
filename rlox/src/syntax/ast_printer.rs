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
            Stmt::Expr(expression, _) => self.visit_expr(expression),
            Stmt::Print(expression, _) => format!(
                "(print {})",
                self.visit_expr(expression)
            ),
            Stmt::Var(name, initializer, _) => match initializer {
                Some(expr) => format!(
                    "(var {} {})",
                    name,
                    self.visit_expr(expr)
                ),
                None => format!(
                    "(var {})",
                    name
                )
            }
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary(left, operator, right, _) => format!(
                "({} {} {})",
                operator.to_string(),
                self.visit_expr(left),
                self.visit_expr(right)
            ),
            Expr::Grouping(expression, _) => format!(
                "({})",
                self.visit_expr(expression)
            ),
            Expr::Literal(value, _) => match value {
                Literal::Number(v) => format!("{}", v),
                Literal::String(v) => format!("{:?}", v),
                Literal::Boolean(v) => format!("{}", v),
                Literal::Nil => String::from("nil")
            },
            Expr::Unary(operator, right, _) => format!(
                "({} {})",
                operator.to_string(),
                self.visit_expr(right)
            ),
            Expr::Variable(name, _) => format!(
                "{}",
                 name
            )
        }
    }
}
