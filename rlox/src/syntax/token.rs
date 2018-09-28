#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token<'a> {
    pub ty: Ty<'a>,
    pub length: usize,
    pub offset: usize
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
