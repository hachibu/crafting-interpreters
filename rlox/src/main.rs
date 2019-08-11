#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod lox;
mod runtime;
mod syntax;

use lox::Lox;

fn main() {
    Lox::new().interpret("
        print ();
        var foo = 0;
        print bar + foo;
    ");
    Lox::new().interactive();
}
