extern crate rustyline;
extern crate yansi;

mod syntax;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use syntax::Scanner;
use yansi::Color;

fn main() {
    let mut editor = Editor::<()>::new();
    let hist_file: &str = ".rlox_history";
    let prompt: &str = ">> ";

    editor.load_history(hist_file).unwrap_or(());

    loop {
        match editor.readline(prompt) {
            Ok(line) => {
                let mut scanner = Scanner::new(&line);
                scanner.source_file = Some("stdin");
                match scanner.scan_tokens() {
                    Ok(tokens) => {
                        for token in tokens {
                            println!("{:?}", token)
                        }
                    },
                    Err(err) => {
                        println!("{}", err)
                    }
                }
                editor.add_history_entry(line.as_ref());
            },
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("{}: {}", Color::Red.paint("ReadlineError"), err);
                break
            }
        }
    }

    editor.save_history(hist_file).unwrap();
}
