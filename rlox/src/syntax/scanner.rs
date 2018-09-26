use syntax::token::*;

#[derive(Debug)]
pub struct Scanner {
    pub source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl_display_trait!(Scanner);

impl Scanner {
    pub fn new(source : &str) -> Scanner {
        Scanner {
            source: String::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::Eof);
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn nth_char(&self, n: usize) -> char {
        self.source.chars().nth(n).unwrap_or('\0')
    }

    fn scan_token(&mut self) -> () {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' =>
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                },
            '=' =>
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                },
            '<' =>
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                },
            '>' =>
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                },
            '/' =>
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
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

    fn advance(&mut self) -> char {
        self.current += 1;
        self.nth_char(self.current - 1)
    }

    fn add_token(&mut self, ty: TokenType) {
        let lexeme = match ty {
            TokenType::Eof => None,
            _ =>
                match self.source.get(self.start..self.current) {
                    Some(s) => Some(String::from(s)),
                    None => None
                }
        };
        let token = Token::new(ty, lexeme, self.line);

        self.tokens.push(token)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.nth_char(self.current) != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.nth_char(self.current)
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.nth_char(self.current + 1)
        }
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("{}", "Unterminated string.");
            return
        }

        self.advance();

        let value = match self.source.get(self.start + 1..self.current - 1) {
            Some(s) => String::from(s),
            None => String::from("")
        };
        self.add_token(TokenType::String(value))
    }

    fn scan_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.source.get(self.start..self.current)
                               .unwrap()
                               .parse::<f64>()
                               .unwrap();
        self.add_token(TokenType::Number(value));
    }
}
