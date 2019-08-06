#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod cli;
mod runtime;
mod syntax;

use syntax::lex::Scanner;
use syntax::ast::{Parser, Printer};

fn main() {
    let source = "1 + 2 * 3";

    match Scanner::new(source).scan_tokens() {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(stmt) => Printer::new().print(&stmt),
            Err(err) => println!("{}", err)
        },
        Err(err) => println!("{}", err)
    }
}
