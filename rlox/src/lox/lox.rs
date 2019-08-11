use runtime::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use syntax::*;
use yansi::Color;

pub struct Lox<'a> {
    editor: Editor<()>,
    prompt: &'a str,
    history_file: &'a str,
    interpreter: Interpreter
}

impl<'a> Lox<'a> {
    pub fn new() -> Lox<'a> {
        Lox {
            editor: Editor::<()>::new(),
            prompt: ">> ",
            history_file: ".rlox_history",
            interpreter: Interpreter::new()
        }
    }

    pub fn interactive(&mut self) {
        self.editor.load_history(self.history_file).unwrap_or(());
        loop {
            match self.editor.readline(self.prompt) {
                Ok(line) => {
                    self.interpret(&line);
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

    pub fn interpret(&mut self, source: &str) {
        match Scanner::new(source).scan_tokens() {
            Ok(tokens) => match Parser::new(tokens, source).parse() {
                Ok(stmts) => match self.interpreter.interpret(stmts, source) {
                    Ok(_) => {},
                    Err(err) => println!("{}", err)
                },
                Err(err) => println!("{}", err)
            },
            Err(err) => println!("{}", err)
        }
    }
}
