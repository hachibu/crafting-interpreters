use syntax::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenTy,
    pub position: Position
}

impl Token {
    pub fn new(ty: TokenTy, position: Position) -> Token {
        Token { ty, position }
    }

    pub fn to_string(&self) -> String {
        match self.ty {
            TokenTy::Minus => "-".to_string(),
            TokenTy::Plus => "+".to_string(),
            TokenTy::LeftParen => "(".to_string(),
            TokenTy::RightParen => ")".to_string(),
            TokenTy::Star => "*".to_string(),
            _ => format!("{:?}", self.ty)
        }
    }
}
