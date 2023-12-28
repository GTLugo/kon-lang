use crate::compiler::token::Token;
use crate::error::KonError;

use super::character_provider::CharacterProvider;

pub struct Lexer<'a> {
  characters: CharacterProvider<'a>,
  eof: bool,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    let characters = CharacterProvider::new(source);
    Self {
      characters,
      eof: false,
    }
  }

  fn unknown_token(&self, c: char) -> Token {
    Token::Invalid { 
      error: KonError::UnknownToken { 
        location: Default::default(), 
        line: self.characters.current_line(), 
        column: self.characters.current_column(), 
        token: c.into() 
      } 
    }
  }

  fn build_token(&mut self, next_character: char) -> Option<Token> {
    let mut lexeme = String::new();
    match next_character {
      letter if letter.is_ascii_alphabetic() || letter == '_' => { // identifiers
        lexeme.push(letter);

        let start_of_lexeme = self.characters.current_column();
        while let Some(c) = self.characters.peek() && (
          c.is_ascii_alphabetic() || c.is_ascii_digit() || c == &'_'
        ) {
          lexeme.push(self.characters.next()?);
        }

        Some(Token::Identifier { line: self.characters.current_line(), column: start_of_lexeme, lexeme })
      }
      digit if digit.is_ascii_digit() => { // numbers
        lexeme.push(digit);

        let start_of_lexeme = self.characters.current_column();
        while let Some(c) = self.characters.peek() && c.is_ascii_digit() {
          lexeme.push(self.characters.next()?);
        }
        
        if let Ok(lexeme) = lexeme.parse::<f64>() {
          Some(Token::Number { line: self.characters.current_line(), column: start_of_lexeme, lexeme })
        } else {
          Some(Token::Invalid { 
            error: KonError::SyntaxError { 
              location: Default::default(), 
              line: self.characters.current_line(), 
              column: self.characters.current_column(),
              message: "Failed to parse number".into(),
            } 
          })
        }
      },
      symbol if symbol.is_ascii_punctuation() => { // symbols
        match symbol {
          '+' => Some(Token::Plus { line: self.characters.current_line(), column: self.characters.current_column() }),
          '-' => {
            if let Some(c) = self.characters.peek() && c == &'>' {
              self.characters.next();
              Some(Token::RightArrow { line: self.characters.current_line(), column: self.characters.current_column() })
            } else {
              Some(Token::Minus { line: self.characters.current_line(), column: self.characters.current_column() })
            }
          },
          '/' => Some(Token::ForwardSlash { line: self.characters.current_line(), column: self.characters.current_column() }),
          '*' => Some(Token::Asterisk { line: self.characters.current_line(), column: self.characters.current_column() }),
          ':' => Some(Token::Colon { line: self.characters.current_line(), column: self.characters.current_column() }),
          '(' => Some(Token::LeftParenthesis { line: self.characters.current_line(), column: self.characters.current_column() }),
          ')' => Some(Token::RightParenthesis { line: self.characters.current_line(), column: self.characters.current_column() }),
          '{' => Some(Token::LeftSquigglyBracket { line: self.characters.current_line(), column: self.characters.current_column() }),
          '}' => Some(Token::RightSquigglyBracket { line: self.characters.current_line(), column: self.characters.current_column() }),
          c => Some(self.unknown_token(c))
        }
      },
      c => Some(self.unknown_token(c)),
    }
  }
}

impl Iterator for Lexer<'_> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    match self.characters.next() {
      Some(ch) => self.build_token(ch),
      None if self.eof => None,
      None => {
        self.eof = true;
        Some(Token::EndOfFile { 
          line: self.characters.current_line(), 
          column: self.characters.current_column() 
        })
      },
    }
  }
}
