use syntax::lex::TokenTy;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenTy,
    pub len: usize,
    pub pos: usize
}
