use std::collections::HashMap;
use syntax::token::*;

#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    keywords: HashMap<&'a str, Ty<'a>>,
    prev: usize,
    curr: usize
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        let tokens = Vec::new();
        let keywords = [
            ("and", Ty::And),
            ("class", Ty::Class),
            ("else", Ty::Else),
            ("false", Ty::False),
            ("for", Ty::For),
            ("fun", Ty::Fun),
            ("if", Ty::If),
            ("nil", Ty::Nil),
            ("or", Ty::Or),
            ("print", Ty::Print),
            ("return", Ty::Return),
            ("super", Ty::Super),
            ("this", Ty::This),
            ("true", Ty::True),
            ("var", Ty::Var),
            ("while", Ty::While)
        ].iter().cloned().collect();
        let (prev, curr) = (0, 0);

        Scanner { source, tokens, keywords, prev, curr }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while self.not_at_end() {
            self.scan_token();
        }
        self.push_token(Ty::Eof);
        self.tokens.clone()
    }

    fn scan_token(&mut self) -> () {
        self.prev = self.curr;
        match self.next() {
            '"' => self.scan_string(),
            '(' => self.push_token(Ty::LeftParen),
            ')' => self.push_token(Ty::RightParen),
            '{' => self.push_token(Ty::LeftBrace),
            '}' => self.push_token(Ty::RightBrace),
            ',' => self.push_token(Ty::Comma),
            '.' => self.push_token(Ty::Dot),
            '-' => self.push_token(Ty::Minus),
            '+' => self.push_token(Ty::Plus),
            ';' => self.push_token(Ty::Semicolon),
            '*' => self.push_token(Ty::Star),
            '!' =>
                if self.next_eq('=') {
                    self.push_token(Ty::BangEqual)
                } else {
                    self.push_token(Ty::Bang)
                },
            '=' =>
                if self.next_eq('=') {
                    self.push_token(Ty::EqualEqual)
                } else {
                    self.push_token(Ty::Equal)
                },
            '<' =>
                if self.next_eq('=') {
                    self.push_token(Ty::LessEqual)
                } else {
                    self.push_token(Ty::Less)
                },
            '>' =>
                if self.next_eq('=') {
                    self.push_token(Ty::GreaterEqual)
                } else {
                    self.push_token(Ty::Greater)
                },
            '/' =>
                if self.next_eq('/') {
                    self.scan_single_line_comment();
                } else if self.next_eq('*') {
                    self.scan_multi_line_comment();
                } else {
                    self.push_token(Ty::Slash)
                },
            c => {
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
        self.scan_until('\n');
    }

    fn scan_multi_line_comment(&mut self) {
        while self.not_at_end() && !(self.peek() == '*' && self.peek_next() == '/') {
            self.next();
        }

        if !(self.next_eq('*') && self.next_eq('/')) {
            println!("Unterminated multi-line comment.");
            return
        }
    }

    fn scan_string(&mut self) {
        self.scan_until('"');

        if !self.next_eq('"') {
            println!("Unterminated string.");
            return
        }

        let value = self.curr_lexeme().trim_matches('"');

        self.push_token(Ty::String(value))
    }

    fn scan_number(&mut self) {
        while self.not_at_end() && self.peek().is_digit(10) {
            self.next();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.next();
            while self.not_at_end() && self.peek().is_digit(10) {
                self.next();
            }
        }

        let value = self.curr_lexeme().parse::<f64>().unwrap();

        self.push_token(Ty::Number(value));
    }

    fn scan_identifier(&mut self) {
        while self.not_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            self.next();
        }

        let value = self.curr_lexeme();
        let token = match self.keywords.get(value) {
            Some(ty) => (*ty).clone(),
            None => Ty::Identifier(value)
        };

        self.push_token(token);
    }

    fn scan_until(&mut self, c: char) {
        while self.not_at_end() && self.peek() != c {
            self.next();
        }
    }

    fn push_token(&mut self, ty: Ty<'a>) {
        let (len, pos) = match ty {
            Ty::Eof => (0, self.source.len()),
            _ => {
                let len = self.curr - self.prev;
                let pos = self.curr - len;
                (len, pos)
            }
        };
        self.tokens.push(Token { ty, len, pos });
    }

    fn not_at_end(&self) -> bool {
        self.curr < self.source.len()
    }

    fn next(&mut self) -> char {
        self.curr += 1;
        self.nth_char(self.curr - 1)
    }

    fn next_eq(&mut self, c: char) -> bool {
        if self.peek() == c {
            self.next();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> char {
        self.nth_char(self.curr)
    }

    fn peek_next(&self) -> char {
        self.nth_char(self.curr + 1)
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap_or('\0')
    }

    fn curr_lexeme(&self) -> &'a str {
        self.source.get(self.prev..self.curr).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_scans_delimeters() {
        let mut scanner = Scanner::new("(){},.;");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::LeftParen, len: 1, pos: 0 },
            Token { ty: Ty::RightParen, len: 1, pos: 1 },
            Token { ty: Ty::LeftBrace, len: 1, pos: 2 },
            Token { ty: Ty::RightBrace, len: 1, pos: 3 },
            Token { ty: Ty::Comma, len: 1, pos: 4 },
            Token { ty: Ty::Dot, len: 1, pos: 5 },
            Token { ty: Ty::Semicolon, len: 1, pos: 6 },
            Token { ty: Ty::Eof, len: 0, pos: 7 }
        ]);
    }

    #[test]
    fn it_scans_arithmetic_operators() {
        let mut scanner = Scanner::new("+-*/");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::Plus, len: 1, pos: 0 },
            Token { ty: Ty::Minus, len: 1, pos: 1 },
            Token { ty: Ty::Star, len: 1, pos: 2 },
            Token { ty: Ty::Slash, len: 1, pos: 3 },
            Token { ty: Ty::Eof, len: 0, pos: 4 }
        ]);
    }

    #[test]
    fn it_scans_logical_operators() {
        let mut scanner = Scanner::new("! != = == > >= <=");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::Bang, len: 1, pos: 0 },
            Token { ty: Ty::BangEqual, len: 2, pos: 2 },
            Token { ty: Ty::Equal, len: 1, pos: 5 },
            Token { ty: Ty::EqualEqual, len: 2, pos: 7 },
            Token { ty: Ty::Greater, len: 1, pos: 10 },
            Token { ty: Ty::GreaterEqual, len: 2, pos: 12 },
            Token { ty: Ty::LessEqual, len: 2, pos: 15 },
            Token { ty: Ty::Eof, len: 0, pos: 17 }
        ]);
    }

    #[test]
    fn it_scans_booleans() {
        let mut scanner = Scanner::new("true false");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::True, len: 4, pos: 0 },
            Token { ty: Ty::False, len: 5, pos: 5 },
            Token { ty: Ty::Eof, len: 0, pos: 10 }
        ]);
    }

    #[test]
    fn it_scans_numbers() {
        let mut scanner = Scanner::new("1 2.0");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::Number(1.0), len: 1, pos: 0 },
            Token { ty: Ty::Number(2.0), len: 3, pos: 2 },
            Token { ty: Ty::Eof, len: 0, pos: 5 }
        ]);
    }

    #[test]
    fn it_scans_strings() {
        let mut scanner = Scanner::new("\"string\"");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::String("string"), len: 8, pos: 0 },
            Token { ty: Ty::Eof, len: 0, pos: 8 }
        ]);
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

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::Eof, len: 0, pos: source.len() }
        ]);
    }

    #[test]
    fn it_scans_identifiers() {
        let mut scanner = Scanner::new("a a0 a_0");

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::Identifier("a"), len: 1, pos: 0 },
            Token { ty: Ty::Identifier("a0"), len: 2, pos: 2 },
            Token { ty: Ty::Identifier("a_0"), len: 3, pos: 5 },
            Token { ty: Ty::Eof, len: 0, pos: 8 }
        ]);
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

        assert_eq!(scanner.scan_tokens(), vec![
            Token { ty: Ty::And, len: 3, pos: 13 },
            Token { ty: Ty::Class, len: 5, pos: 29 },
            Token { ty: Ty::Else, len: 4, pos: 47 },
            Token { ty: Ty::False, len: 5, pos: 64 },
            Token { ty: Ty::For, len: 3, pos: 82 },
            Token { ty: Ty::Fun, len: 3, pos: 98 },
            Token { ty: Ty::If, len: 2, pos: 114 },
            Token { ty: Ty::Nil, len: 3, pos: 129 },
            Token { ty: Ty::Or, len: 2, pos: 145 },
            Token { ty: Ty::Print, len: 5, pos: 160 },
            Token { ty: Ty::Return, len: 6, pos: 178 },
            Token { ty: Ty::Super, len: 5, pos: 197 },
            Token { ty: Ty::This, len: 4, pos: 215 },
            Token { ty: Ty::True, len: 4, pos: 232 },
            Token { ty: Ty::Var, len: 3, pos: 249 },
            Token { ty: Ty::While, len: 5, pos: 265 },
            Token { ty: Ty::Eof, len: 0, pos: 279 }
        ]);
    }
}
