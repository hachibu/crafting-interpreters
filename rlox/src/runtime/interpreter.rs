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
            Expr::Binary(_left, _operator, _right) => {
                //Object left = evaluate(expr.left);
                //Object right = evaluate(expr.right);
                //
                //switch (expr.operator.type) {
                //    case MINUS:
                //        return (double)left - (double)right;
                //    case SLASH:
                //        return (double)left / (double)right;
                //    case STAR:
                //        return (double)left * (double)right;
                //    case PLUS:
                //        if (left instanceof Double && right instanceof Double) {
                //            return (double)left + (double)right;
                //        }
                //        if (left instanceof String && right instanceof String) {
                //            return (String)left + (String)right;
                //        }
                //    case GREATER:
                //        return (double)left > (double)right;
                //    case GREATER_EQUAL:
                //        return (double)left >= (double)right;
                //    case LESS:
                //        return (double)left < (double)right;
                //    case LESS_EQUAL:
                //        return (double)left <= (double)right;
                //    case BANG_EQUAL: return !isEqual(left, right);
                //    case EQUAL_EQUAL: return isEqual(left, right);
                //}
                //
                LoxObject::Nil
            }
        }
    }
}
