use std::mem::discriminant;
use syntax::*;

pub struct Parser {
    tokens: Vec<Token>,
    source: String,
    pub source_file: Option<String>,
    curr: usize,
    prev: usize,
    error: Option<String>
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: &str) -> Parser {
        Parser {
            tokens,
            source: String::from(source),
            source_file: None,
            curr: 0,
            prev: 0,
            error: None
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<Stmt>>, SyntaxError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        match self.error {
            Some(ref message) => Err(
                SyntaxError::new(
                    &message,
                    &self.source,
                    &self.source_file,
                    self.prev
                )
            ),
            None => Ok(statements)
        }
    }

    fn statement(&mut self) -> Box<Stmt> {
        if self.match_1(TokenTy::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Box<Stmt> {
        let expr = self.expression();
        self.consume(
            TokenTy::Semicolon,
            "Expected `;` after expression."
        );
        Box::new(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Box<Stmt> {
        let expr = self.expression();
        self.consume(
            TokenTy::Semicolon,
            "Expected `;` after expression."
        );
        Box::new(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.match_2(TokenTy::BangEqual, TokenTy::EqualEqual) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.addition();

        while self.match_4(
            TokenTy::Greater,
            TokenTy::GreaterEqual,
            TokenTy::Less,
            TokenTy::LessEqual
        ) {
            let operator = self.previous();
            let right = self.addition();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn addition(&mut self) -> Box<Expr> {
        let mut expr = self.multiplication();

        while self.match_2(TokenTy::Minus, TokenTy::Plus) {
            let operator = self.previous();
            let right = self.multiplication();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn multiplication(&mut self) -> Box<Expr> {
        let mut expr = self.unary();

        while self.match_2(TokenTy::Slash, TokenTy::Star) {
            let operator = self.previous();
            let right = self.unary();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_2(TokenTy::Bang, TokenTy::Minus) {
            let operator = self.previous();
            let right = self.unary();

            Box::new(Expr::Unary(operator, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_1(TokenTy::False) {
            Box::new(Expr::Literal(Literal::Boolean(false)))
        }
        else if self.match_1(TokenTy::True) {
            Box::new(Expr::Literal(Literal::Boolean(true)))
        }
        else if self.match_1(TokenTy::Nil) {
            Box::new(Expr::Literal(Literal::Nil))
        }
        else if self.match_2(
            TokenTy::Number(0.0),
            TokenTy::String(String::from(""))
        ) {
            match self.previous().ty {
                TokenTy::Number(v) => Box::new(Expr::Literal(Literal::Number(v))),
                TokenTy::String(v) => Box::new(Expr::Literal(Literal::String(v))),
                _ => panic!()
            }
        }
        else if self.match_1(TokenTy::LeftParen) {
            let expr = self.expression();
            self.consume(
                TokenTy::RightParen,
                "Expected `)` after expression."
            );
            Box::new(Expr::Grouping(expr))
        } else {
            panic!()
        }
    }

    fn consume(&mut self, ty: TokenTy, message: &str) -> Token {
        if !self.check(&ty) {
            self.error = Some(message.to_string());
        }
        self.advance()
    }

    fn match_many(&mut self, tys: &[TokenTy]) -> bool {
        for ty in tys {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        return false
    }

    fn match_1(&mut self, a: TokenTy) -> bool {
        self.match_many(&[a])
    }

    fn match_2(&mut self, a: TokenTy, b: TokenTy) -> bool {
        self.match_many(&[a, b])
    }

    fn match_4(&mut self, a: TokenTy, b: TokenTy, c: TokenTy, d: TokenTy) -> bool {
        self.match_many(&[a, b, c, d])
    }

    fn check(&mut self, token_ty: &TokenTy) -> bool {
        if self.is_at_end() {
            false
        } else {
            discriminant(&self.peek().ty) == discriminant(&token_ty)
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.curr += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.error.is_some() || self.peek().ty == TokenTy::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.curr).unwrap().clone()
    }

    fn previous(&mut self) -> Token {
        let previous = self.tokens.get(self.curr - 1).unwrap().clone();
        self.prev = previous.pos;
        previous
    }
}
