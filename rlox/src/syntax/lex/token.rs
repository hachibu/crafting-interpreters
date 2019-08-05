use syntax::lex::TokenTy;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenTy,
    pub len: usize,
    pub pos: usize
}

impl Token {
    pub fn new(ty: TokenTy, len: usize, pos: usize) -> Token {
        Token { ty, len, pos }
    }

    pub fn to_string(&self) -> String {
        match &self.ty {
            TokenTy::Minus => String::from("-"),
            TokenTy::Star => String::from("*"),
            ty => format!("{:?}", ty)
        }
    }
}
