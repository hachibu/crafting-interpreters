use runtime::*;
use syntax::*;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&mut self, s: &Stmt) -> LoxObject {
        self.visit_stmt(s)
    }

    pub fn is_truthy(&mut self, o: &LoxObject) -> bool {
        match o {
            LoxObject::Boolean(v) => *v,
            LoxObject::Nil => false,
            _ => true
        }
    }

    pub fn is_equal(&mut self, a: &LoxObject, b: &LoxObject) -> bool {
        match (a, b) {
            (LoxObject::Nil, LoxObject::Nil) => true,
            (LoxObject::Nil, _) => false,
            (LoxObject::Boolean(a), LoxObject::Boolean(b)) => a == b,
            (LoxObject::Number(a), LoxObject::Number(b)) => a == b,
            (LoxObject::String(a), LoxObject::String(b)) => a == b,
            (_, _) => false
        }
    }
}

impl Visitor<LoxObject> for Interpreter {
    fn visit_stmt(&mut self, s: &Stmt) -> LoxObject {
        match s {
            Stmt::Expr(expression) => {
                self.visit_expr(expression)
            },
            Stmt::Print(expression) => {
                self.visit_expr(expression)
            }
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> LoxObject {
        match e {
            Expr::Grouping(expression) => {
                self.visit_expr(expression)
            },
            Expr::Literal(value) => match value {
                Literal::Number(v) => LoxObject::Number(*v),
                Literal::String(v) => LoxObject::String(v.to_string()),
                Literal::Boolean(v) => LoxObject::Boolean(*v),
                Literal::Nil => LoxObject::Nil
            },
            Expr::Unary(operator, right) => {
                let object = self.visit_expr(right);

                match operator.ty {
                    TokenTy::Bang => {
                        LoxObject::Boolean(!self.is_truthy(&object))
                    },
                    TokenTy::Minus => match object {
                        LoxObject::Number(v) => LoxObject::Number(-v),
                        _ => object
                    },
                    _ => object
                }
            },
            Expr::Binary(left, operator, right) => {
                let lhs = self.visit_expr(left);
                let rhs = self.visit_expr(right);

                match operator.ty {
                    TokenTy::Minus => match (lhs, rhs) {
                        (LoxObject::Number(a), LoxObject::Number(b)) => {
                            LoxObject::Number(a + b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::Slash => match (lhs, rhs) {
                        (LoxObject::Number(a), LoxObject::Number(b)) => {
                            LoxObject::Number(a / b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::Star => match (lhs, rhs) {
                        (LoxObject::Number(a), LoxObject::Number(b)) => {
                            LoxObject::Number(a * b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::Plus => match (lhs, rhs) {
                        (LoxObject::Number(a), LoxObject::Number(b)) => {
                            LoxObject::Number(a + b)
                        },
                        (LoxObject::String(a), LoxObject::String(b)) => {
                            LoxObject::String([a, b].join(""))
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::Greater => match (lhs, rhs) {
                        (LoxObject::Boolean(a), LoxObject::Boolean(b)) => {
                            LoxObject::Boolean(a > b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::GreaterEqual => match (lhs, rhs) {
                        (LoxObject::Boolean(a), LoxObject::Boolean(b)) => {
                            LoxObject::Boolean(a >= b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::Less => match (lhs, rhs) {
                        (LoxObject::Boolean(a), LoxObject::Boolean(b)) => {
                            LoxObject::Boolean(a < b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::LessEqual => match (lhs, rhs) {
                        (LoxObject::Boolean(a), LoxObject::Boolean(b)) => {
                            LoxObject::Boolean(a <= b)
                        },
                        (_, _) => LoxObject::Nil
                    },
                    TokenTy::BangEqual => {
                        LoxObject::Boolean(!self.is_equal(&lhs, &rhs))
                    },
                    TokenTy::EqualEqual => {
                        LoxObject::Boolean(self.is_equal(&lhs, &rhs))
                    },
                    _ => LoxObject::Nil
                }
            }
        }
    }
}
