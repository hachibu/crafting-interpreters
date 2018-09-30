mod syntax;

use syntax::Scanner;

fn main() {
    let mut scanner = Scanner::new("
        var x = 1;
        var y = $;
    ");
    scanner.source_file = Some("src/main.lox");

    match scanner.scan_tokens() {
        Ok(tokens) => println!("{:?}", tokens),
        Err(error) => println!("{}", error)
    }
}
