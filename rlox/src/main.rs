extern crate rustyline;
extern crate yansi;

mod ast;
mod cli;
mod syntax;

use ast::{Expr};
use syntax::{Token, Ty};

fn main() {
    let expr = Expr::Binary(
        Expr::Unary(
            Token { ty: Ty::Minus, len: 1, pos: 0 },
            Expr::NumberLiteral(123.0).into_boxed()
        ).into_boxed(),
        Token { ty: Ty::Star, len: 1, pos: 2 },
        Expr::Grouping(
            Expr::NumberLiteral(45.67).into_boxed()
        ).into_boxed()
    ).into_boxed();

    println!("{:#?}", expr);
}
