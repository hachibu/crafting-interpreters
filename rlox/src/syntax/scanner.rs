use std::collections::HashMap;
use lox::*;
use syntax::*;

pub struct Scanner {
    tokens: Vec<Token>,
    keywords: HashMap<&'static str, TokenTy>,
    source: String,
    pub source_file: Option<String>,
    curr: usize,
    prev: usize,
    error: Option<String>
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let keywords = [
            ("and", TokenTy::And),
            ("class", TokenTy::Class),
            ("else", TokenTy::Else),
            ("false", TokenTy::False),
            ("for", TokenTy::For),
            ("fun", TokenTy::Fun),
            ("if", TokenTy::If),
            ("nil", TokenTy::Nil),
            ("or", TokenTy::Or),
            ("print", TokenTy::Print),
            ("return", TokenTy::Return),
            ("super", TokenTy::Super),
            ("this", TokenTy::This),
            ("true", TokenTy::True),
            ("var", TokenTy::Var),
            ("while", TokenTy::While)
        ].iter().cloned().collect();

        Scanner {
            source: String::from(source),
            source_file: None,
            keywords,
            tokens: Vec::new(),
            curr: 0,
            prev: 0,
            error: None
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.at_end() {
            self.scan_token();
        }

        match self.error {
            Some(ref message) => Err(
                LoxError::new(
                    LoxErrorTy::Syntax,
                    &message,
                    &self.source,
                    &self.source_file,
                    self.prev
                )
            ),
            None => {
                self.push_token(TokenTy::Eof);
                Ok(self.tokens.clone())
            }
        }
    }

    fn scan_token(&mut self) -> () {
        self.prev = self.curr;
        match self.next() {
            '"' => self.scan_string(),
            '(' => self.push_token(TokenTy::LeftParen),
            ')' => self.push_token(TokenTy::RightParen),
            '{' => self.push_token(TokenTy::LeftBrace),
            '}' => self.push_token(TokenTy::RightBrace),
            ',' => self.push_token(TokenTy::Comma),
            '.' => self.push_token(TokenTy::Dot),
            '-' => self.push_token(TokenTy::Minus),
            '+' => self.push_token(TokenTy::Plus),
            ';' => self.push_token(TokenTy::Semicolon),
            '*' => self.push_token(TokenTy::Star),
            '!' =>
                if self.next_eq('=') {
                    self.push_token(TokenTy::BangEqual)
                } else {
                    self.push_token(TokenTy::Bang)
                },
            '=' =>
                if self.next_eq('=') {
                    self.push_token(TokenTy::EqualEqual)
                } else {
                    self.push_token(TokenTy::Equal)
                },
            '<' =>
                if self.next_eq('=') {
                    self.push_token(TokenTy::LessEqual)
                } else {
                    self.push_token(TokenTy::Less)
                },
            '>' =>
                if self.next_eq('=') {
                    self.push_token(TokenTy::GreaterEqual)
                } else {
                    self.push_token(TokenTy::Greater)
                },
            '/' =>
                if self.next_eq('/') {
                    self.scan_single_line_comment();
                } else if self.next_eq('*') {
                    self.scan_multi_line_comment();
                } else {
                    self.push_token(TokenTy::Slash)
                },
            c =>
                if c.is_whitespace() {
                    return
                } else if c.is_digit(10) {
                    self.scan_number()
                } else if c.is_alphabetic() {
                    self.scan_identifier()
                } else {
                    self.stop("Unexpected character.");
                }
        }
    }

    fn scan_single_line_comment(&mut self) {
        self.skip_until(|c| c == '\n');
    }

    fn scan_multi_line_comment(&mut self) {
        while !self.at_end() &&
              !(self.peek_eq('*') && self.peek_next_eq('/')) {
            self.next();
        }

        if !(self.next_eq('*') && self.next_eq('/')) {
            self.stop("Unterminated multi-line comment. Expected `*/`");
        }
    }

    fn scan_string(&mut self) {
        self.skip_until(|c| c == '"');

        if !self.next_eq('"') {
            self.stop("Unterminated string. Expected `\"`");
        }

        let value = self.curr_lexeme().trim_matches('"');
        let token = TokenTy::String(value.to_string());

        self.push_token(token);
    }

    fn scan_number(&mut self) {
        self.skip_while(|c| c.is_digit(10));

        if self.peek_eq('.') && self.peek_next().is_digit(10) {
            self.next();
            self.skip_while(|c| c.is_digit(10));
        }

        let value = self.curr_lexeme().parse::<f64>().unwrap();
        let token = TokenTy::Number(value);

        self.push_token(token);
    }

    fn scan_identifier(&mut self) {
        while !self.at_end() &&
              (self.peek().is_alphanumeric() || self.peek_eq('_')) {
            self.next();
        }

        let value = self.curr_lexeme();
        let token = match self.keywords.get(value) {
            Some(ty) => (*ty).clone(),
            None => TokenTy::Identifier(value.to_string())
        };

        self.push_token(token);
    }

    fn push_token(&mut self, ty: TokenTy) {
        let (len, pos) = match ty {
            TokenTy::Eof => (0, self.source.len()),
            _ => {
                let len = self.curr - self.prev;
                let pos = self.curr - len;
                (len, pos)
            }
        };

        self.tokens.push(Token { ty, len, pos });
    }

    fn at_end(&self) -> bool {
        self.error.is_some() || self.curr >= self.source.len()
    }

    fn stop(&mut self, message: &str) {
        self.error = Some(message.to_string());
    }

    fn next(&mut self) -> char {
        self.curr += 1;
        self.nth_char(self.curr - 1)
    }

    fn next_eq(&mut self, c: char) -> bool {
        if self.peek_eq(c) {
            self.next();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> char {
        self.nth_char(self.curr)
    }

    fn peek_eq(&self, c: char) -> bool {
        self.peek() == c
    }

    fn peek_next(&self) -> char {
        self.nth_char(self.curr + 1)
    }

    fn peek_next_eq(&self, c: char) -> bool {
        self.peek_next() == c
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap_or('\0')
    }

    fn curr_lexeme(&self) -> &str {
        self.source.get(self.prev..self.curr).unwrap_or("")
    }

    fn skip_until<F: Fn(char) -> bool>(&mut self, pred: F) {
        while !self.at_end() && !pred(self.peek()) {
            self.next();
        }
    }

    fn skip_while<F: Fn(char) -> bool>(&mut self, pred: F) {
        while !self.at_end() && pred(self.peek()) {
            self.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_scans_delimeters() {
        let mut scanner = Scanner::new("(){},.;");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::LeftParen, len: 1, pos: 0 },
            Token { ty: TokenTy::RightParen, len: 1, pos: 1 },
            Token { ty: TokenTy::LeftBrace, len: 1, pos: 2 },
            Token { ty: TokenTy::RightBrace, len: 1, pos: 3 },
            Token { ty: TokenTy::Comma, len: 1, pos: 4 },
            Token { ty: TokenTy::Dot, len: 1, pos: 5 },
            Token { ty: TokenTy::Semicolon, len: 1, pos: 6 },
            Token { ty: TokenTy::Eof, len: 0, pos: 7 }
        ]));
    }

    #[test]
    fn it_scans_arithmetic_operators() {
        let mut scanner = Scanner::new("+-*/");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::Plus, len: 1, pos: 0 },
            Token { ty: TokenTy::Minus, len: 1, pos: 1 },
            Token { ty: TokenTy::Star, len: 1, pos: 2 },
            Token { ty: TokenTy::Slash, len: 1, pos: 3 },
            Token { ty: TokenTy::Eof, len: 0, pos: 4 }
        ]));
    }

    #[test]
    fn it_scans_logical_operators() {
        let mut scanner = Scanner::new("! != = == > >= <=");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::Bang, len: 1, pos: 0 },
            Token { ty: TokenTy::BangEqual, len: 2, pos: 2 },
            Token { ty: TokenTy::Equal, len: 1, pos: 5 },
            Token { ty: TokenTy::EqualEqual, len: 2, pos: 7 },
            Token { ty: TokenTy::Greater, len: 1, pos: 10 },
            Token { ty: TokenTy::GreaterEqual, len: 2, pos: 12 },
            Token { ty: TokenTy::LessEqual, len: 2, pos: 15 },
            Token { ty: TokenTy::Eof, len: 0, pos: 17 }
        ]));
    }

    #[test]
    fn it_scans_booleans() {
        let mut scanner = Scanner::new("true false");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::True, len: 4, pos: 0 },
            Token { ty: TokenTy::False, len: 5, pos: 5 },
            Token { ty: TokenTy::Eof, len: 0, pos: 10 }
        ]));
    }

    #[test]
    fn it_scans_numbers() {
        let mut scanner = Scanner::new("1 2.0");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::Number(1.0), len: 1, pos: 0 },
            Token { ty: TokenTy::Number(2.0), len: 3, pos: 2 },
            Token { ty: TokenTy::Eof, len: 0, pos: 5 }
        ]));
    }

    #[test]
    fn it_scans_strings() {
        let mut scanner = Scanner::new("\"string\"");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::String(String::from("string")), len: 8, pos: 0 },
            Token { ty: TokenTy::Eof, len: 0, pos: 8 }
        ]));
    }

    #[test]
    fn it_scans_comments() {
        let source = "
            // single-line comment
            /*
             * multi-line comment
             * */
        ";
        let mut scanner = Scanner::new(source);

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::Eof, len: 0, pos: source.len() }
        ]));
    }

    #[test]
    fn it_scans_identifiers() {
        let mut scanner = Scanner::new("a a0 a_0");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::Identifier(String::from("a")), len: 1, pos: 0 },
            Token { ty: TokenTy::Identifier(String::from("a0")), len: 2, pos: 2 },
            Token { ty: TokenTy::Identifier(String::from("a_0")), len: 3, pos: 5 },
            Token { ty: TokenTy::Eof, len: 0, pos: 8 }
        ]));
    }

    #[test]
    fn it_scans_keywords() {
        let mut scanner = Scanner::new("
            and
            class
            else
            false
            for
            fun
            if
            nil
            or
            print
            return
            super
            this
            true
            var
            while
        ");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token { ty: TokenTy::And, len: 3, pos: 13 },
            Token { ty: TokenTy::Class, len: 5, pos: 29 },
            Token { ty: TokenTy::Else, len: 4, pos: 47 },
            Token { ty: TokenTy::False, len: 5, pos: 64 },
            Token { ty: TokenTy::For, len: 3, pos: 82 },
            Token { ty: TokenTy::Fun, len: 3, pos: 98 },
            Token { ty: TokenTy::If, len: 2, pos: 114 },
            Token { ty: TokenTy::Nil, len: 3, pos: 129 },
            Token { ty: TokenTy::Or, len: 2, pos: 145 },
            Token { ty: TokenTy::Print, len: 5, pos: 160 },
            Token { ty: TokenTy::Return, len: 6, pos: 178 },
            Token { ty: TokenTy::Super, len: 5, pos: 197 },
            Token { ty: TokenTy::This, len: 4, pos: 215 },
            Token { ty: TokenTy::True, len: 4, pos: 232 },
            Token { ty: TokenTy::Var, len: 3, pos: 249 },
            Token { ty: TokenTy::While, len: 5, pos: 265 },
            Token { ty: TokenTy::Eof, len: 0, pos: 279 }
        ]));
    }
}
