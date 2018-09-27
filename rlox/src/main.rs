mod syntax;

use syntax::*;

fn main() {
    let scanner = Scanner::new("1
        /*
        * multi-line comment
        * */
        \"string\"
        1
        true
        class
        foo
        == <= !
        // single-line comment
        /* multi-line
         * comment
         * */
    ");

    for token in scanner.scan_tokens() {
        println!("{:?}", token);
    }
}
