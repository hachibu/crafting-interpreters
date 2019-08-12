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
            source: source.to_string(),
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
            Some(ref message) => {
                Err(
                    LoxError::new(
                        LoxErrorTy::Syntax,
                        &message,
                        &self.source,
                        &self.source_file,
                        Position::new(1, self.curr - 1)
                    )
                )
            },
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
                    self.stop(&format!(
                        "Unexpected character `{}`.",
                        c
                    ));
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
        let (length, offset) = match ty {
            TokenTy::Eof => (0, self.source.len()),
            _ => {
                let length = self.curr - self.prev;
                let offset = self.curr - length;
                (length, offset)
            }
        };

        self.tokens.push(Token::new(ty, Position::new(length, offset)));
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
            Token::new(TokenTy::LeftParen, Position::new(1, 0)),
            Token::new(TokenTy::RightParen, Position::new(1, 1)),
            Token::new(TokenTy::LeftBrace, Position::new(1,  2)),
            Token::new(TokenTy::RightBrace, Position::new(1, 3)),
            Token::new(TokenTy::Comma, Position::new(1, 4)),
            Token::new(TokenTy::Dot, Position::new(1, 5)),
            Token::new(TokenTy::Semicolon, Position::new(1, 6)),
            Token::new(TokenTy::Eof, Position::new(0, 7))
        ]));
    }

    #[test]
    fn it_scans_arithmetic_operators() {
        let mut scanner = Scanner::new("+-*/");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token::new(TokenTy::Plus, Position::new(1, 0)),
            Token::new(TokenTy::Minus, Position::new(1, 1)),
            Token::new(TokenTy::Star, Position::new(1, 2)),
            Token::new(TokenTy::Slash, Position::new(1, 3)),
            Token::new(TokenTy::Eof, Position::new(0, 4))
        ]));
    }

    #[test]
    fn it_scans_logical_operators() {
        let mut scanner = Scanner::new("! != = == > >= <=");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token::new(TokenTy::Bang, Position::new(1, 0)),
            Token::new(TokenTy::BangEqual, Position::new(2, 2)),
            Token::new(TokenTy::Equal, Position::new(1, 5)),
            Token::new(TokenTy::EqualEqual, Position::new(2, 7)),
            Token::new(TokenTy::Greater, Position::new(1, 10)),
            Token::new(TokenTy::GreaterEqual, Position::new(2, 12)),
            Token::new(TokenTy::LessEqual, Position::new(2, 15)),
            Token::new(TokenTy::Eof, Position::new(0, 17))
        ]));
    }

    #[test]
    fn it_scans_booleans() {
        let mut scanner = Scanner::new("true false");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token::new(TokenTy::True, Position::new(4, 0)),
            Token::new(TokenTy::False, Position::new(5, 5)),
            Token::new(TokenTy::Eof, Position::new(0, 10))
        ]));
    }

    #[test]
    fn it_scans_numbers() {
        let mut scanner = Scanner::new("1 2.0");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token::new(TokenTy::Number(1.0), Position::new(1, 0)),
            Token::new(TokenTy::Number(2.0), Position::new(3, 2)),
            Token::new(TokenTy::Eof, Position::new(0, 5))
        ]));
    }

    #[test]
    fn it_scans_strings() {
        let mut scanner = Scanner::new("\"string\"");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token::new(TokenTy::String("string".to_string()), Position::new(8, 0)),
            Token::new(TokenTy::Eof, Position::new(0, 8))
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
            Token::new(TokenTy::Eof, Position::new(0, source.len()))
        ]));
    }

    #[test]
    fn it_scans_identifiers() {
        let mut scanner = Scanner::new("a a0 a_0");

        assert_eq!(scanner.scan_tokens(), Ok(vec![
            Token::new(TokenTy::Identifier("a".to_string()), Position::new(1, 0)),
            Token::new(TokenTy::Identifier("a0".to_string()), Position::new(2, 2)),
            Token::new(TokenTy::Identifier("a_0".to_string()), Position::new(3, 5)),
            Token::new(TokenTy::Eof, Position::new(0, 8))
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
            Token::new(TokenTy::And, Position::new(3, 13)),
            Token::new(TokenTy::Class, Position::new(5, 29)),
            Token::new(TokenTy::Else, Position::new(4, 47)),
            Token::new(TokenTy::False, Position::new(5, 64)),
            Token::new(TokenTy::For, Position::new(3, 82)),
            Token::new(TokenTy::Fun, Position::new(3, 98)),
            Token::new(TokenTy::If, Position::new(2, 114)),
            Token::new(TokenTy::Nil, Position::new(3, 129)),
            Token::new(TokenTy::Or, Position::new(2, 145)),
            Token::new(TokenTy::Print, Position::new(5, 160)),
            Token::new(TokenTy::Return, Position::new(6, 178)),
            Token::new(TokenTy::Super, Position::new(5, 197)),
            Token::new(TokenTy::This, Position::new(4, 215)),
            Token::new(TokenTy::True, Position::new(4, 232)),
            Token::new(TokenTy::Var, Position::new(3, 249)),
            Token::new(TokenTy::While, Position::new(5, 265)),
            Token::new(TokenTy::Eof, Position::new(0, 279))
        ]));
    }
}
