#![allow(dead_code)]

extern crate rustyline;
extern crate yansi;

mod cli;
mod runtime;
mod syntax;

use cli::*;

fn main() {
    Shell::new().evaluate("1 + 2 * 3");
}
