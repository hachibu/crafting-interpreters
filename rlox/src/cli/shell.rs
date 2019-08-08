use rustyline::error::ReadlineError;
use rustyline::Editor;
use syntax::*;
use yansi::Color;

pub struct Shell<'a> {
    editor: Editor<()>,
    prompt: &'a str,
    history_file: &'a str
}

impl<'a> Shell<'a> {
    pub fn new() -> Shell<'a> {
        Shell {
            editor: Editor::<()>::new(),
            prompt: ">> ",
            history_file: ".rlox_history"
        }
    }

    pub fn run(&mut self) {
        self.editor.load_history(self.history_file).unwrap_or(());

        loop {
            match self.editor.readline(self.prompt) {
                Ok(line) => {
                    let mut scanner = Scanner::new(&line);
                    scanner.source_file = Some("stdin");
                    match scanner.scan_tokens() {
                        Ok(tokens) => match Parser::new(tokens).parse() {
                            Ok(stmt) => AstPrinter::new().print(&stmt),
                            Err(err) => println!("{}", err)
                        },
                        Err(err) => println!("{}", err)
                    }
                    self.editor.add_history_entry(line);
                },
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("{}: {}", Color::Red.paint("ReadlineError"), err);
                    break
                }
            }
        }

        self.editor.save_history(self.history_file).unwrap();
    }
}
