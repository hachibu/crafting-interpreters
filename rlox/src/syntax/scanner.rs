use std::collections::HashMap;
use syntax::token::*;

#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    pub source: &'a str,
    tokens: Vec<Token<'a>>,
    keywords: HashMap<&'a str, Ty<'a>>,
    prev: usize,
    curr: usize
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
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
        ].iter().cloned().collect();

        Scanner {
            source: source,
            tokens: Vec::new(),
            keywords: keywords,
            prev: 0,
            curr: 0
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token<'a>> {
        while !self.eof() {
            self.prev = self.curr;
            self.scan_token();
        }
        self.add_token(Ty::Eof);
        &self.tokens
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '"' => self.scan_string(),
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
                    self.scan_single_line_comment();
                } else if self.match_char('*') {
                    self.scan_multi_line_comment();
                } else {
                    self.add_token(Ty::Slash)
                },
            _ => {
                if c.is_whitespace() {
                    return
                } else if c.is_digit(10) {
                    self.scan_number()
                } else if c.is_alphabetic() {
                    self.scan_identifier()
                } else {
                    println!("Unexpected character: {}.", c);
                    return
                }
            }
        }
    }

    fn scan_single_line_comment(&mut self) {
        while !self.eof() && !self.peek_char('\n') {
            self.advance();
        }
    }

    fn scan_multi_line_comment(&mut self) {
        while !self.eof() && !(self.peek() == '*' && self.peek_nth(1) == '/') {
            self.advance();
        }

        if !(self.match_char('*') && self.match_char('/')) {
            println!("Unterminated multi-line comment.");
            return
        }
    }

    fn scan_string(&mut self) {
        while !self.eof() && !self.peek_char('"') {
            self.advance();
        }

        if !self.match_char('"') {
            println!("Unterminated string.");
            return
        }

        let value = self.lexeme().trim_matches('"');

        self.add_token(Ty::String(value))
    }

    fn scan_number(&mut self) {
        while !self.eof() && self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek_char('.') && self.peek_nth(1).is_digit(10) {
            self.advance();
            while !self.eof() && self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.lexeme().parse::<f64>().unwrap();

        self.add_token(Ty::Number(value));
    }

    fn scan_identifier(&mut self) {
        while !self.eof() && self.peek().is_alphanumeric() {
            self.advance();
        }

        let value = self.lexeme();
        let token = match self.keywords.get(value) {
            Some(ty) => (*ty).clone(),
            None => Ty::Identifier(value)
        };

        self.add_token(token);
    }

    fn add_token(&mut self, ty: Ty<'a>) {
        let length = self.curr - self.prev;
        let offset = self.curr;

        self.tokens.push(Token { ty, length, offset });
    }

    fn lexeme(&self) -> &'a str {
        self.source.get(self.prev..self.curr).unwrap_or("")
    }

    fn eof(&self) -> bool {
        self.curr >= self.source.len()
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        self.curr += 1;
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
       self.peek() == c
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
