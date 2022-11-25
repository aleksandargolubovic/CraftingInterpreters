use crate::token::{Token, TokenType};

pub struct Scanner {
  source: String,
  pub tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: u64,
}

impl Scanner {
 pub fn new (source: String) -> Self {
  Scanner {
    source,
    tokens: Vec::new(),
    start: 0,
    current: 0,
    line: 1,
  }
 }

 pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (u64, &'static str)> {
  while !self.is_at_end() {
    self.start = self.current;
    let result = self.scan_token()?;
  }

  self.tokens.push(
    Token::new(TokenType::EOF, String::from(""), self.line));

  Ok(self.tokens.clone())
 }

 fn scan_token(&mut self) -> Result<(), (u64, &'static str)>{
  let c = self.advance();
  match c {
    '(' => self.add_token(TokenType::LEFT_PAREN),
    ')' => self.add_token(TokenType::RIGHT_PAREN),
    '{' => self.add_token(TokenType::LEFT_BRACE),
    '}' => self.add_token(TokenType::RIGHT_BRACE),
    ',' => self.add_token(TokenType::COMMA),
    '.' => self.add_token(TokenType::DOT),
    '-' => self.add_token(TokenType::MINUS),
    '+' => self.add_token(TokenType::PLUS),
    ';' => self.add_token(TokenType::SEMICOLON),
    '*' => self.add_token(TokenType::STAR),
    '!' => {
      let token_type = if self.match_char('=') {
        TokenType::BANG_EQUAL
      } else {
        TokenType::BANG
      };
      self.add_token(token_type);
    },
    '=' => {
      let token_type = if self.match_char('=') {
        TokenType::EQUAL_EQUAL
      } else {
        TokenType::EQUAL
      };
      self.add_token(token_type);
    },
    '<' => {
      let token_type = if self.match_char('=') {
        TokenType::LESS_EQUAL
      } else {
        TokenType::LESS
      };
      self.add_token(token_type);
    },
    '>' => {
      let token_type = if self.match_char('=') {
        TokenType::GREATER_EQUAL
      } else {
        TokenType::GREATER
      };
      self.add_token(token_type);
    },
    '/' => {
      if self.match_char('/') {
        while self.peek() != '\n' && !self.is_at_end() {
          self.advance();
        }
      } else {
        self.add_token(TokenType::SLASH);
      }
    },
    ' ' | '\r' | '\t' => (),
    '\n' => self.line += 1,
    _ => { return Err((self.line, "Unexpected character!")) },
  }
  Ok(())
 }

 fn is_at_end(&self) -> bool {
  self.current >= self.source.len()
 }

 fn advance(&mut self) -> char {
  self.current += 1;
  self.source.chars().nth(self.current - 1).unwrap()
 }

 fn add_token(&mut self, token_type: TokenType) {
  let text: String = self.source.chars()
    .skip(self.start)
    .take(self.current - self.start)
    .collect();

  self.tokens.push(Token::new(token_type, text, self.line));
 }

 fn match_char(&mut self, expected: char) -> bool{
  if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
    return false;
  }
  self.current += 1;
  true
 }

 fn peek(&self) -> char {
  if self.is_at_end() {
    return '\0';
  }
  self.source.chars().nth(self.current).unwrap()
 }
}