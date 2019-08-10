#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod lox;
mod runtime;
mod syntax;

use lox::Lox;

fn main() {
    Lox::new().interpret("
        var a = 1;
        var b = 2;
        print a + b;
    ");
}
