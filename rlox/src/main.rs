mod syntax;

use syntax::{Token, TokenType, Literal};

fn main() {
    let token = Token {
        token_type: TokenType::Number,
        lexeme: String::from("1"),
        literal: Literal::Number(1),
        line: 1
    };
    println!("{}", token.to_string());
}
