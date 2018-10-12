extern crate rustyline;
extern crate yansi;

mod cli;
mod syntax;

fn main() {
    cli::Shell::new().run()
}
