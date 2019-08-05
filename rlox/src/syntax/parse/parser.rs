use std::mem::{discriminant};
use syntax::lex::{Token, TokenTy};
use syntax::ast::*;

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

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        let mut expr = self.comparison();

        while self.match_types(&[
            TokenTy::BangEqual,
            TokenTy::EqualEqual
        ]) {
            let operator = self.previous();
            let right = self.comparison();

            expr = BinaryExpr::new(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> Box<Expr> {
        let mut expr = self.addition();

        while self.match_types(&[
            TokenTy::Greater,
            TokenTy::GreaterEqual,
            TokenTy::Less,
            TokenTy::LessEqual
        ]) {
            let operator = self.previous();
            let right = self.addition();

            expr = BinaryExpr::new(expr, operator, right);
        }

        expr
    }

    fn addition(&mut self) -> Box<Expr> {
        let mut expr = self.multiplication();

        while self.match_types(&[
            TokenTy::Minus,
            TokenTy::Plus
        ]) {
            let operator = self.previous();
            let right = self.multiplication();

            expr = BinaryExpr::new(expr, operator, right);
        }

        expr
    }

    fn multiplication(&mut self) -> Box<Expr> {
        let mut expr = self.unary();

        while self.match_types(&[
            TokenTy::Slash,
            TokenTy::Star
        ]) {
            let operator = self.previous();
            let right = self.unary();

            expr = BinaryExpr::new(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_types(&[
            TokenTy::Bang,
            TokenTy::Minus
        ]) {
            let operator = self.previous();
            let right = self.unary();

            UnaryExpr::new(operator, right)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_types(&[TokenTy::False]) {
            LiteralExpr::new(Literal::Boolean(false))
        }
        else if self.match_types(&[TokenTy::True]) {
            LiteralExpr::new(Literal::Boolean(true))
        }
        else if self.match_types(&[TokenTy::Nil]) {
            LiteralExpr::new(Literal::Nil)
        }
        else if self.match_types(&[
            TokenTy::Number(0.0),
            TokenTy::String(String::from(""))
        ]) {
            let previous = self.previous();
            match previous.ty {
                TokenTy::Number(v) => LiteralExpr::new(Literal::Number(v)),
                TokenTy::String(v) => LiteralExpr::new(Literal::String(v)),
                _ => panic!()
            }
        }
        else if self.match_types(&[TokenTy::LeftParen]) {
            let expr = self.expression();
            self.consume(
                TokenTy::RightParen,
                String::from("Expect ')' after expression.")
            );
            GroupingExpr::new(expr)
        } else {
            panic!()
        }
    }

    fn consume(&self, _token_ty: TokenTy, _message: String) -> Token {
        unimplemented!()
    }

    fn match_types(&mut self, types: &[TokenTy]) -> bool {
        for ty in types {
            if self.check(ty.clone()) {
                self.advance();
                return true;
            }
        }
        return false
    }

    fn check(&mut self, token_ty: TokenTy) -> bool {
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