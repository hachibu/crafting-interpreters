use syntax::lex::Token;
use syntax::ast::Visitor;

pub enum Literal {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NilLiteral,
}

pub trait Expr {
    fn accept(&self, v: &mut Visitor<String>) -> String;
}

macro_rules! impl_expr {
    (
        $id:ident {
            $( $attr_name:ident : $attr_type:ty ),*
        },
        $method:ident
    ) => {
        pub struct $id {
            $( pub $attr_name : $attr_type ),*
        }

        impl Expr for $id {
            fn accept(&self, v: &mut Visitor<String>) -> String {
                v.$method(self)
            }
        }
    }
}

impl_expr!(
    BinaryExpr {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    visit_binary_expr
);

impl_expr!(
    GroupingExpr {
        expression: Box<Expr>
    },
    visit_grouping_expr
);

impl_expr!(
    UnaryExpr {
        operator: Token,
        right: Box<Expr>
    },
    visit_unary_expr
);

impl_expr!(
    LiteralExpr {
        value: Literal
    },
    visit_literal_expr
);
