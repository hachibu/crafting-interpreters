use position::Position;
use std::collections::HashMap;
use syntax::token::*;

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source: String,
    tokens: Vec<Token>,
    keywords: HashMap<&'a str, Ty>,
    prev: usize,
    curr: usize,
    position: Position
}

impl<'a> Scanner<'a> {
    pub fn new(source : &str) -> Scanner {
        let keywords = [
            ("and",    Ty::And),
            ("class",  Ty::Class),
            ("else",   Ty::Else),
            ("false",  Ty::False),
            ("for",    Ty::For),
            ("fun",    Ty::Fun),
            ("if",     Ty::If),
            ("nil",    Ty::Nil),
            ("or",     Ty::Or),
            ("print",  Ty::Print),
            ("return", Ty::Return),
            ("super",  Ty::Super),
            ("this",   Ty::This),
            ("true",   Ty::True),
            ("var",    Ty::Var),
            ("while",  Ty::While)
        ];

        let scanner = Scanner {
            source: String::from(source),
            tokens: Vec::new(),
            keywords: keywords.iter().cloned().collect(),
            prev: 0,
            curr: 0,
            position: Position { line: 1, column: 1 }
        };

        scanner
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.eof() {
            self.prev = self.curr;
            self.scan_token();
        }
        self.advance();
        self.add_token(Ty::Eof);
        self.tokens.clone()
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token(Ty::LeftParen),
            ')' => self.add_token(Ty::RightParen),
            '{' => self.add_token(Ty::LeftBrace),
            '}' => self.add_token(Ty::RightBrace),
            ',' => self.add_token(Ty::Comma),
            '.' => self.add_token(Ty::Dot),
            '-' => self.add_token(Ty::Minus),
            '+' => self.add_token(Ty::Plus),
            ';' => self.add_token(Ty::Semicolon),
            '*' => self.add_token(Ty::Star),
            '!' =>
                if self.match_char('=') {
                    self.add_token(Ty::BangEqual)
                } else {
                    self.add_token(Ty::Bang)
                },
            '=' =>
                if self.match_char('=') {
                    self.add_token(Ty::EqualEqual)
                } else {
                    self.add_token(Ty::Equal)
                },
            '<' =>
                if self.match_char('=') {
                    self.add_token(Ty::LessEqual)
                } else {
                    self.add_token(Ty::Less)
                },
            '>' =>
                if self.match_char('=') {
                    self.add_token(Ty::GreaterEqual)
                } else {
                    self.add_token(Ty::Greater)
                },
            '/' =>
                if self.match_char('/') {
                    while !self.peek_char('\n') {
                        self.advance();
                    }
                } else {
                    self.add_token(Ty::Slash)
                },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => {
                self.position.column = 0;
                self.position.line += 1
            },
            '"' => self.scan_string(),
            c => {
                if c.is_digit(10) {
                    self.scan_number()
                } else {
                    println!("{}: \"{}\"", "Unexpected character", c)
                }
            }
        }
    }

    fn scan_string(&mut self) {
        while !self.peek_char('"') {
            if self.peek_char('\n') {
                self.position.column = 0;
                self.position.line += 1
            }
            self.advance();
        }

        if self.eof() {
            println!("{}", "Unterminated string.");
            return
        }

        self.advance();

        let value = match self.source.get(self.prev + 1..self.curr - 1) {
            Some(s) => String::from(s),
            None => String::from("")
        };
        self.add_token(Ty::String(value))
    }

    fn scan_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek_char('.') && self.peek_nth(1).is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.source.get(self.prev..self.curr).unwrap()
                               .parse::<f64>().unwrap();
        self.add_token(Ty::Number(value));
    }

    fn add_token(&mut self, ty: Ty) {
        let lexeme = match self.source.get(self.prev..self.curr) {
            Some(value) => {
                self.position.column -= value.len() - 1;
                String::from(value)
            },
            None => String::from("")
        };
        self.tokens.push(Token { ty, lexeme, position: self.position })
    }

    fn eof(&self) -> bool {
        self.curr >= self.source.len()
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        self.curr += 1;
        self.position.column += 1;
        self.nth_char(self.curr - 1)
    }

    fn peek_nth(&self, n: usize) -> char {
        if self.curr + n >= self.source.len() {
            '\0'
        } else {
            self.nth_char(self.curr + n)
        }
    }

    fn peek(&self) -> char {
        self.peek_nth(0)
    }

    fn peek_char(&self, c: char) -> bool {
        !self.eof() && self.peek() == c
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.peek_char(c) {
            self.curr += 1;
            true
        } else {
            false
        }
    }
}
