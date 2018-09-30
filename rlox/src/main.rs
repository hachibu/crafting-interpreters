mod syntax;

use syntax::Scanner;

fn main() {
    let mut scanner = Scanner::new("var language = \"lox\";");

    println!("{:#?}", scanner.scan_tokens());
}
