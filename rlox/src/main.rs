#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod cli;
mod runtime;
mod syntax;

use runtime::*;
use syntax::*;

fn main() {
    let source = "1 + 2 * 3";

    match Scanner::new(source).scan_tokens() {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(stmt) => {
                Printer::new().print(&stmt);
                println!("{:#?}", Interpreter::new().evaluate(&stmt))
            },
            Err(err) => println!("{}", err)
        },
        Err(err) => println!("{}", err)
    }
}
