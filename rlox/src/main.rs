extern crate rustyline;
extern crate yansi;

mod cli;
#[macro_use]
mod syntax;

use syntax::lex::*;
use syntax::ast::{Expr, Stmt};

fn main() {
    let stmt = Stmt::Expr(
        expr_binary!(
            expr_unary!(token!(TokenTy::Minus), expr_number_literal!(123.0)),
            token!(TokenTy::Star),
            expr_grouping!(expr_number_literal!(45.67))
        )
    );

    println!("{:#?}", stmt);
}
