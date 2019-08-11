use syntax::*;

pub struct AstPosition;

impl AstPosition {
    pub fn new() -> AstPosition {
        AstPosition {}
    }

    pub fn get_stmt_position(&mut self, s: &Stmt) -> Position {
        self.visit_stmt(s)
    }

    pub fn get_expr_position(&mut self, e: &Expr) -> Position {
        self.visit_expr(e)
    }
}

impl Visitor<Position> for AstPosition {
    fn visit_stmt(&mut self, s: &Stmt) -> Position {
        match s {
            Stmt::Expr(expression, position) => {
                Position::new(
                    position.length + self.visit_expr(expression).length,
                    position.offset
                )
            },
            Stmt::Print(expression, position) => {
                Position::new(
                    position.length + self.visit_expr(expression).length,
                    position.offset
                )
            },
            Stmt::Var(_name, initializer, position) => {
                match initializer {
                    Some(expr) => Position::new(
                        position.length + self.visit_expr(expr).length,
                        position.offset
                    ),
                    None => position.clone()
                }
            }
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> Position {
        match e {
            Expr::Binary(left, _, right, position) => {
                Position::new(
                    position.length +
                    self.visit_expr(left).length +
                    self.visit_expr(right).length,
                    position.offset
                )
            },
            Expr::Grouping(expression, position) => {
                Position::new(
                    position.length + self.visit_expr(expression).length,
                    position.offset
                )
            },
            Expr::Unary(_, right, position) => {
                Position::new(
                    position.length + self.visit_expr(right).length,
                    position.offset
                )
            },
            Expr::Variable(_, position) => position.clone(),
            Expr::Literal(_, position) => position.clone()
        }
    }
}
