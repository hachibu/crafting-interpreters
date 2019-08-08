#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod lox;
mod runtime;
mod syntax;

use lox::Lox;

fn main() {
    Lox::new().interpret("1 + 2 * 3");
}
