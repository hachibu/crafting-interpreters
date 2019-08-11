use lox::*;
use runtime::*;
use syntax::*;

pub struct Interpreter {
    environment: Environment,
    error: Option<String>,
    source: String,
    pub source_file: Option<String>,
    prev: usize
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
            error: None,
            source: "".to_string(),
            source_file: None,
            prev: 0
        }
    }

    pub fn interpret(
        &mut self,
        stmts: Vec<Box<Stmt>>,
        source: &str
    ) -> Result<(), LoxError> {
        self.source = source.to_string();

        let mut iter = stmts.iter();
        let mut last = None;

        while self.error.is_none() {
            match iter.next() {
                None => {
                    break;
                },
                Some(stmt) => {
                    last = Some(stmt);
                    self.visit_stmt(&stmt);
                }
            }
        }

        match self.error {
            Some(ref message) => {
                let prev = match last {
                    Some(stmt) => match **stmt {
                        Stmt::Expr(_, source_map) => source_map.offset,
                        Stmt::Print(_, source_map) => source_map.offset,
                        Stmt::Var(_, _, source_map) => source_map.offset,
                    },
                    None => 0
                };

                Err(
                    LoxError::new(
                        LoxErrorTy::Runtime,
                        &message,
                        &self.source,
                        &self.source_file,
                        prev
                    )
                )
            },
            None => {
                Ok(())
            }
        }
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
            Stmt::Expr(expression, _) => {
                self.visit_expr(expression)
            },
            Stmt::Print(expression, _) => {
                let value = self.visit_expr(expression);
                println!("{}", value);
                LoxObject::Nil
            },
            Stmt::Var(name, initializer, _) => {
                let value = match initializer {
                    Some(expr) => self.visit_expr(expr),
                    None => LoxObject::Nil
                };
                self.environment.define(name.to_string(), value);
                LoxObject::Nil
            }
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> LoxObject {
        match e {
            Expr::Grouping(expression, _) => {
                self.visit_expr(expression)
            },
            Expr::Literal(value, _) => match value {
                Literal::Number(v) => LoxObject::Number(*v),
                Literal::String(v) => LoxObject::String(v.to_string()),
                Literal::Boolean(v) => LoxObject::Boolean(*v),
                Literal::Nil => LoxObject::Nil
            },
            Expr::Unary(operator, right, _) => {
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
            Expr::Binary(left, operator, right, _) => {
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
            },
            Expr::Variable(name, _) => {
                match self.environment.get(name.to_string()) {
                    Some(value) => {
                        value.clone()
                    },
                    None => {
                        self.error = Some(format!(
                            "Undefined variable `{}`.",
                            name
                        ));
                        LoxObject::Nil
                    }
                }
            }
        }
    }
}
