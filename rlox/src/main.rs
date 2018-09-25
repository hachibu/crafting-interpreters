mod syntax;

use syntax::*;

fn main() {
    let scanner = Scanner::new("var lanugage = \"rlox\";");

    for token in scanner.scan_tokens() {
        println!("{:#?}", token);
    }
}
