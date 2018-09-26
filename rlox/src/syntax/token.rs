#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Option<String>,
    pub position: Position
}

impl_display_trait!(Token);

impl Token {
    pub fn new(ty: TokenType, lexeme: Option<String>, position: Position) -> Token {
        Token { ty, lexeme, position }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TokenType {
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

impl_display_trait!(TokenType);

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

impl_display_trait!(Position);

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position { line, column }
    }
}
