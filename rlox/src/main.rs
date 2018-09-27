mod syntax;

use syntax::*;

fn main() {
    let scanner = Scanner::new("
        // comment
        (()){}
        !*+-/=<> <= ==
        \"rlox\"
        1 1.2
        class
        hello
    ");

    for token in scanner.scan_tokens() {
        println!("{:?}", token);
    }
}
