use crate::token::{Token, TokenType};
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
  "and" => TokenType::AND,
  "class" => TokenType::CLASS,
  "else" => TokenType::ELSE,
  "false" => TokenType::FALSE,
  "for" => TokenType::FOR,
  "fun" => TokenType::FUN,
  "if" => TokenType::IF,
  "nil" => TokenType::NIL,
  "or" => TokenType::OR,
  "print" => TokenType::PRINT,
  "return" => TokenType::RETURN,
  "super" => TokenType::SUPER,
  "this" => TokenType::THIS,
  "true" => TokenType::TRUE,
  "var" => TokenType::VAR,
  "while" => TokenType::WHILE,
};

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
    '\"' => self.make_string()?,
    _ => {
      if c.is_ascii_digit() {
        self.make_number()?;
      } else if c.is_alphabetic() || c == '_' || c == '-' {
        self.make_identifier()?;
      } else {
        return Err((self.line, "Unexpected character!"))
      }
    },
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

 fn peek_next(&self) -> char {
  if self.current + 1 >= self.source.len() {
    return '\0';
  }
  self.source.chars().nth(self.current + 1).unwrap()
 }

 fn make_string(&mut self) -> Result<(), (u64, &'static str)> {
  while self.peek() != '\"' && !self.is_at_end() {
    if self.peek() == '\n' {
      self.line += 1;
    }
    self.advance();
  }

  if self.is_at_end() {
    return Err((self.line, "Unterminated string"));
  };

  self.advance();

  let value = self.source.chars()
    .skip(self.start + 1)
    .take(self.current - self.start - 2)
    .collect();

  self.add_token(TokenType::STRING(value));
  Ok(())
 }

 fn make_number(&mut self) -> Result<(), (u64, &'static str)> {
  while self.peek().is_ascii_digit() {
    self.advance();
  }
  if self.peek() == '.' && self.peek_next().is_ascii_digit() {
    self.advance();
    while self.peek().is_ascii_digit() {
      self.advance();
    }
  }
  let value = self.source.chars()
    .skip(self.start)
    .take(self.current - self.start)
    .collect::<String>();

  match value.parse::<f64>() {
    Ok(n) => self.add_token(TokenType::NUMBER(n)),
    Err(e) => return Err((self.line, "Unrecognized number")),
  };
  Ok(())
 }

 fn make_identifier(&mut self) -> Result<(), (u64, &'static str)> {
  while self.peek().is_alphanumeric() {
    self.advance();
  }
  let value = self.source.chars()
    .skip(self.start)
    .take(self.current - self.start)
    .collect::<String>();

  match KEYWORDS.get(&value) {
    None => self.add_token(TokenType::IDENTIFIER(value)),
    Some(n) => self.add_token(n.clone()),
  }
  Ok(())
 }
}