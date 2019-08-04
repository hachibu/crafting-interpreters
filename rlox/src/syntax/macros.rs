macro_rules! token {
    ($ty:expr) => (Token { ty: $ty, len: 0, pos: 0 });
    ($ty:expr, $len:expr) => (Token { ty: $ty, len: $len, pos: 0 });
    ($ty:expr, $len:expr, $pos:expr) => (Token { ty: $ty, len: $len, pos: $pos });
}

macro_rules! expr_binary {
    ($a:expr, $b:expr, $c:expr) => (syntax::ast::BinaryExpr {
        left: $a,
        operator: $b,
        right: $c
    });
}

macro_rules! expr_number_literal {
    ($a:expr) => (Box::new(syntax::ast::LiteralExpr {
        value: syntax::ast::Literal::Number($a)
    }));
}

macro_rules! expr_grouping {
    ($a:expr) => (Box::new(syntax::ast::GroupingExpr { expression: $a }));
}

macro_rules! expr_unary {
    ($a:expr, $b:expr) => (Box::new(syntax::ast::UnaryExpr { operator: $a, right: $b }));
}
