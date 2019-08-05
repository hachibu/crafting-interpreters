#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod cli;
mod syntax;

use syntax::lex::*;
use syntax::ast::*;

fn main() {
    let expr = BinaryExpr::new(
        UnaryExpr::new(
            Token::new(TokenTy::Minus, 1, 0),
            LiteralExpr::new(Literal::Number(123.0))
        ),
        Token::new(TokenTy::Star, 1, 7),
        GroupingExpr::new(LiteralExpr::new(Literal::Number(45.67)))
    );

    Printer::new().print(expr);
}
