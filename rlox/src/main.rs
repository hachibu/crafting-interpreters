#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod lox;
mod runtime;
mod syntax;

use lox::Lox;

fn main() {
    Lox::new().interpret("
        printAst 1 + 2;
        print 1 + 2;
    ");
}
