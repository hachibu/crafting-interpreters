#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod lox;
mod runtime;
mod syntax;

use lox::Lox;

fn main() {
    Lox::new().interpret("
        print \"one\";
        print true;
        print 2 + 1;
    ");
}
