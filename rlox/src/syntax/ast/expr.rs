use syntax::lex::Token;
use syntax::ast::Visitor;

pub trait Expr {
    fn accept(&self, v: &mut Visitor<String>) -> String;
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>
}

impl Expr for BinaryExpr {
    fn accept(&self, v: &mut Visitor<String>) -> String{
        v.visit_binary_expr(self)
    }
}

pub struct GroupingExpr {
    pub expression: Box<Expr>
}

impl Expr for GroupingExpr {
    fn accept(&self, v: &mut Visitor<String>) -> String{
        v.visit_grouping_expr(self)
    }
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>
}

impl Expr for UnaryExpr {
    fn accept(&self, v: &mut Visitor<String>) -> String{
        v.visit_unary_expr(self)
    }
}

pub enum Literal {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NilLiteral,
}

pub struct LiteralExpr {
    pub value: Literal
}

impl LiteralExpr {
    pub fn to_string(&self) -> String {
        match &self.value {
            Literal::NumberLiteral(v) => format!("{:}", v),
            Literal::StringLiteral(v) => format!("{:}", v),
            Literal::BooleanLiteral(v) => format!("{:}", v),
            Literal::NilLiteral => String::from("nil"),
        }
    }
}

impl Expr for LiteralExpr {
    fn accept(&self, v: &mut Visitor<String>) -> String {
        v.visit_literal_expr(self)
    }
}
