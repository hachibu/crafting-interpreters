#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub ty: Ty<'a>,
    pub len: usize,
    pub pos: usize
}

#[derive(Clone, Debug, PartialEq)]
pub enum Ty<'a> {
  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  Comma,
  Dot,
  Minus,
  Plus,
  Semicolon,
  Slash,
  Star,
  Bang,
  BangEqual,
  Equal,
  EqualEqual,
  Greater,
  GreaterEqual,
  Less,
  LessEqual,
  Identifier(&'a str),
  String(&'a str),
  Number(f64),
  And,
  Class,
  Else,
  False,
  Fun,
  For,
  If,
  Nil,
  Or,
  Print,
  Return,
  Super,
  This,
  True,
  Var,
  While,
  Eof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_impl_partial_eq() {
        let (len, pos) = (0, 0);

        assert_eq!(
            Token { ty: Ty::Eof, len, pos },
            Token { ty: Ty::Eof, len, pos }
        );
        assert_eq!(Ty::Eof, Ty::Eof);

        assert_ne!(
            Token { ty: Ty::Eof, len, pos },
            Token { ty: Ty::Nil, len, pos }
        );
        assert_ne!(Ty::Eof, Ty::Nil);
    }
}
