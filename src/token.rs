use std::fmt;


#[derive(Clone, Debug)]
pub enum TokenType {
  // Single-character tokens.
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  SLASH,
  STAR,

  // One or two character tokens.
  BANG,
  BANG_EQUAL,
  EQUAL,
  EQUAL_EQUAL,
  GREATER,
  GREATER_EQUAL,
  LESS,
  LESS_EQUAL,

  // Literals.
  IDENTIFIER(String),
  STRING(String),
  NUMBER(f64),

  // Keywords.
  AND,
  CLASS,
  ELSE,
  FALSE,
  FUN,
  FOR,
  IF,
  NIL,
  OR,
  PRINT,
  RETURN,
  SUPER,
  THIS,
  TRUE,
  VAR,
  WHILE,

  EOF,
}

#[derive(Clone)]
pub struct Token {
  type_: TokenType,
  lexeme: String,
  // literal: Option<String>,
  line: u64,
}

impl Token {
  pub fn new(type_: TokenType, lexeme: String, line: u64) -> Self {
    Token {
      type_,
      lexeme,
      line
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} {:?} {}", self.type_, self.lexeme, self.line)
  }
}