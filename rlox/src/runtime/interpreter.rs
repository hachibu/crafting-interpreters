use syntax::*;

#[derive(Debug)]
pub enum Object {
    Nil
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn evaluate(&mut self, s: &Stmt) -> Object {
        self.visit_stmt(s)
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_stmt(&mut self, s: &Stmt) -> Object {
        match s {
            Stmt::Expr(expression) => self.visit_expr(expression)
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> Object {
        match e {
            Expr::Binary(_left, _operator, _right) => Object::Nil,
            Expr::Grouping(_expression) => Object::Nil,
            Expr::Literal(value) => match value {
                Literal::Number(_v) => Object::Nil,
                Literal::String(_v) => Object::Nil,
                Literal::Boolean(_v) => Object::Nil,
                Literal::Nil => Object::Nil
            },
            Expr::Unary(_operator, _right) => Object::Nil,
        }
    }
}
