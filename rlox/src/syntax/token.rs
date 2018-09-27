use position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: Ty,
    pub lexeme: Lexeme,
    pub position: Position
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Ty {
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
  Identifier(String),
  String(String),
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

pub type Lexeme = String;
