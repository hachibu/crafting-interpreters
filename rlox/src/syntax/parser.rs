use std::mem::discriminant;
use syntax::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Box<Stmt>, SyntaxError> {
        Ok(Box::new(Stmt::Expr(self.expression())))
    }

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.match_ty(&[
            TokenTy::BangEqual,
            TokenTy::EqualEqual
        ]) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.addition();

        while self.match_ty(&[
            TokenTy::Greater,
            TokenTy::GreaterEqual,
            TokenTy::Less,
            TokenTy::LessEqual
        ]) {
            let operator = self.previous();
            let right = self.addition();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn addition(&mut self) -> Box<Expr> {
        let mut expr = self.multiplication();

        while self.match_ty(&[
            TokenTy::Minus,
            TokenTy::Plus
        ]) {
            let operator = self.previous();
            let right = self.multiplication();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn multiplication(&mut self) -> Box<Expr> {
        let mut expr = self.unary();

        while self.match_ty(&[
            TokenTy::Slash,
            TokenTy::Star
        ]) {
            let operator = self.previous();
            let right = self.unary();

            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_ty(&[
            TokenTy::Bang,
            TokenTy::Minus
        ]) {
            let operator = self.previous();
            let right = self.unary();

            Box::new(Expr::Unary(operator, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_ty(&[TokenTy::False]) {
            Box::new(Expr::Literal(Literal::Boolean(false)))
        }
        else if self.match_ty(&[TokenTy::True]) {
            Box::new(Expr::Literal(Literal::Boolean(true)))
        }
        else if self.match_ty(&[TokenTy::Nil]) {
            Box::new(Expr::Literal(Literal::Nil))
        }
        else if self.match_ty(&[
            TokenTy::Number(0.0),
            TokenTy::String(String::from(""))
        ]) {
            let previous = self.previous();
            match previous.ty {
                TokenTy::Number(v) => Box::new(Expr::Literal(Literal::Number(v))),
                TokenTy::String(v) => Box::new(Expr::Literal(Literal::String(v))),
                _ => panic!()
            }
        }
        else if self.match_ty(&[TokenTy::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenTy::RightParen,
                String::from("Expect ')' after expression.")
            );
            Box::new(Expr::Grouping(expr))
        } else {
            panic!()
        }
    }

    fn consume(&self, _token_ty: TokenTy, _message: String) -> Token {
        unimplemented!()
    }

    fn match_ty(&mut self, types: &[TokenTy]) -> bool {
        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        return false
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
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ty == TokenTy::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }
}
