mod syntax;

use std::io::{self, Write};
use syntax::Scanner;

fn main() -> io::Result<()> {
    loop {
        io::stdout().write(b">>> ")?;
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut scanner = Scanner::new(&input);

                scanner.source_file = Some("stdin");

                match scanner.scan_tokens() {
                    Ok(tokens) => {
                        for token in tokens {
                            println!("{:?}", token)
                        }
                    },
                    Err(error) => println!("{}", error)
                }
            }
            Err(error) => println!("{}", error),
        }
    }
}
