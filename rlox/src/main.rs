mod syntax;

use syntax::*;

fn main() {
    let scanner = Scanner::new("
        // single-line comment
        /* multi-line
         * comment
         * */
    ");

    for token in scanner.scan_tokens() {
        println!("{:?}", token);
    }
}
