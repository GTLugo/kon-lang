use foxy_utils::types::handle::Handle;

use super::{
  character_provider::CharacterProvider,
  token::{Literal, Symbol},
};
use crate::{
  error::{error_handler::ErrorHandler, InterpreterError},
  interpreter::token::Token,
};

pub struct Lexer {
  error_handler: Handle<ErrorHandler>,
}

impl Lexer {
  pub fn new(error_handler: Handle<ErrorHandler>) -> Self {
    Self { error_handler }
  }

  pub fn lex(&mut self, source: &str) -> Vec<Token> {
    let mut characters = CharacterProvider::new(source);

    let mut tokens = Vec::default();
    while let Some(token) = self.build_token(&mut characters) {
      if let Token::Invalid { error } = &token {
        self.error_handler.get_mut().push(error.clone());
      }
      tokens.push(token);
    }

    tokens.push(Token::EndOfFile {
      line: characters.current_line(),
      column: characters.current_column(),
    });

    tokens
  }

  fn build_token(&mut self, characters: &mut CharacterProvider) -> Option<Token> {
    let next_character = characters.next()?;
    let mut lexeme = String::new();
    match next_character {
      letter if letter.is_ascii_alphabetic() || letter == '_' => {
        let start_of_lexeme = characters.current_column();

        // identifiers
        lexeme.push(letter);
        lexeme.push_str(&Self::read_lexeme_while(characters, |c| c.is_ascii_alphanumeric() || c == &'_'));

        return if let Some(reserved_word) = Token::reserved_word(&lexeme, characters.current_line(), start_of_lexeme) {
          Some(reserved_word)
        } else {
          Some(Token::Literal {
            line: characters.current_line(),
            column: start_of_lexeme,
            literal: Literal::Identifier { lexeme },
          })
        };
      }
      digit if digit.is_ascii_digit() => {
        let start_of_lexeme = characters.current_column();

        // numbers
        lexeme.push(digit);
        lexeme.push_str(&Self::read_lexeme_while(characters, |c| c.is_ascii_digit()));

        return if let Ok(lexeme) = lexeme.parse::<f64>() {
          Some(Token::Literal {
            line: characters.current_line(),
            column: start_of_lexeme,
            literal: Literal::Number { lexeme },
          })
        } else {
          Some(Token::Invalid {
            error: InterpreterError::SyntaxError {
              line: characters.current_line(),
              column: characters.current_column(),
              message: "Failed to parse number".into(),
            },
          })
        };
      }
      symbol if symbol.is_ascii_punctuation() => {
        // symbols
        match symbol {
          ';' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::Semicolon,
            })
          }
          ',' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::Comma,
            })
          }
          '.' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::Period,
            })
          }
          ':' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::Colon,
            })
          }
          '!' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::ExclamationPointEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::ExclamationPoint,
              })
            };
          }
          '=' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::DoubleEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::Equals,
              })
            };
          }
          '+' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::PlusEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::Plus,
              })
            };
          }
          '-' => {
            return if Self::next_char_is(characters, '>') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::RightArrow,
              })
            } else if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::MinusEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::Minus,
              })
            };
          }
          '/' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::ForwardSlashEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::ForwardSlash,
              })
            };
          }
          '*' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::AsteriskEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::Asterisk,
              })
            };
          }
          '^' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::CaretEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::Caret,
              })
            };
          }
          '&' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::AmpersandEquals,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::Ampersand,
              })
            };
          }
          '(' => {
            return if Self::next_char_is(characters, ')') {
              Some(Token::Literal {
                line: characters.current_line(),
                column: characters.current_column(),
                literal: Literal::Void,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::LeftParenthesis,
              })
            };
          }
          ')' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::RightParenthesis,
            })
          }
          '{' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::LeftCurlyBracket,
            })
          }
          '}' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::RightCurlyBracket,
            })
          }
          '<' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::LeftAngledBracket,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::LeftAngledBracketEquals,
              })
            };
          }
          '>' => {
            return if Self::next_char_is(characters, '=') {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::RightAngledBracket,
              })
            } else {
              Some(Token::Symbol {
                line: characters.current_line(),
                column: characters.current_column(),
                symbol: Symbol::RightAngledBracketEquals,
              })
            };
          }
          '[' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::LeftSquareBracket,
            })
          }
          ']' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::RightSquareBracket,
            })
          }
          '\'' => {
            return Some(Token::Symbol {
              line: characters.current_line(),
              column: characters.current_column(),
              symbol: Symbol::Apostrophe,
            })
          }
          '\"' => {
            let start_of_lexeme = characters.current_column();

            let is_end_of_string = Self::next_char_is(characters, '\"');
            if let Some(_) = characters.peek()
              && !is_end_of_string
            {
              while let Some(c) = characters.peek() {
                if *c != '\"' {
                  lexeme.push(characters.next_with_spaces().unwrap());
                } else {
                  characters.next().unwrap();

                  return Some(Token::Literal {
                    line: characters.current_line(),
                    column: start_of_lexeme,
                    literal: Literal::String { lexeme },
                  });
                }
              }
            }

            return Some(Token::Invalid {
              error: InterpreterError::UnterminatedString {
                line: characters.current_line(),
                column: characters.current_column(),
              },
            });
          }
          _ => {}
        }
      }
      _ => {}
    }

    Some(Token::Invalid {
      error: InterpreterError::UnknownToken {
        line: characters.current_line(),
        column: characters.current_column(),
        token: next_character.into(),
      },
    })
  }

  fn next_char_is(characters: &mut CharacterProvider, character: char) -> bool {
    if let Some(c) = characters.peek()
      && c == &character
    {
      characters.next();
      true
    } else {
      false
    }
  }

  fn read_lexeme_while(characters: &mut CharacterProvider, condition: impl Fn(&char) -> bool) -> String {
    let mut lexeme = String::new();
    while let Some(c) = characters.peek()
      && condition(c)
    {
      lexeme.push(characters.next().unwrap());
    }
    lexeme
  }
}
