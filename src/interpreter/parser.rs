use foxy_utils::types::handle::Handle;

use super::{
  grammar::{Expression, SyntaxTree},
  token::{Keyword, Literal, Symbol, Token},
  token_provider::{Next, TokenProvider},
};
use crate::error::{error_handler::ErrorHandler, InterpreterError};

pub struct Parser {
  error_handler: Handle<ErrorHandler>,
}

impl Parser {
  pub fn new(error_handler: Handle<ErrorHandler>) -> Self {
    Self { error_handler }
  }

  pub fn parse(&mut self, tokens: &[Token]) -> SyntaxTree {
    let mut tokens = TokenProvider::new(tokens);
    let root = self.expression(&mut tokens).unwrap_or_else(|_| Expression::Invalid);

    let next = tokens.peek().cloned();
    match next {
      Next::Token(t) => {
        self.error_handler.get_mut().push(InterpreterError::ParseError {
          line: t.line(),
          column: t.column(),
          message: format!("Expected expression but got `{}`", t),
        });
        SyntaxTree {
          root,
          eof: Token::EndOfFile {
            line: t.line(),
            column: t.column() + 1,
          },
        }
      }
      Next::EndOfFile { line, column } => SyntaxTree {
        root,
        eof: Token::EndOfFile { line, column },
      },
      Next::EndOfStream { line, column } => SyntaxTree {
        root,
        eof: Token::EndOfFile { line, column },
      },
    }
  }

  fn expression(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    self.equality(tokens)
  }

  fn equality(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    let mut expression = self.comparison(tokens)?;

    while matches!(
      tokens.peek(),
      Next::Token(Token::Symbol {
        symbol: Symbol::DoubleEquals | Symbol::ExclamationPointEquals,
        ..
      })
    ) {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.comparison(tokens)?);
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    Ok(expression)
  }

  fn comparison(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    let mut expression = self.term(tokens)?;

    while matches!(
      tokens.peek(),
      Next::Token(Token::Symbol {
        symbol: Symbol::LeftAngledBracket
          | Symbol::RightAngledBracket
          | Symbol::LeftAngledBracketEquals
          | Symbol::RightAngledBracketEquals,
        ..
      })
    ) {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.term(tokens)?);
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    Ok(expression)
  }

  fn term(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    let mut expression = self.factor(tokens)?;

    while matches!(
      tokens.peek(),
      Next::Token(Token::Symbol {
        symbol: Symbol::Plus | Symbol::Minus,
        ..
      })
    ) {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.factor(tokens)?);
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    Ok(expression)
  }

  fn factor(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    let mut expression = self.power(tokens)?;

    while matches!(
      tokens.peek(),
      Next::Token(Token::Symbol {
        symbol: Symbol::Asterisk | Symbol::ForwardSlash,
        ..
      })
    ) {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.power(tokens)?);
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    Ok(expression)
  }

  fn power(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    let mut expression = self.unary(tokens)?;

    while matches!(
      tokens.peek(),
      Next::Token(Token::Symbol {
        symbol: Symbol::Caret,
        ..
      })
    ) {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.unary(tokens)?);
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    Ok(expression)
  }

  fn unary(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    if matches!(
      tokens.peek(),
      Next::Token(Token::Symbol {
        symbol: Symbol::ExclamationPoint | Symbol::Minus,
        ..
      })
    ) {
      if let Next::Token(operator) = tokens.next().cloned() {
        let operand = Box::new(self.unary(tokens)?);
        return Ok(Expression::Unary { operator, operand });
      }
    }

    self.primary(tokens)
  }

  fn primary(&mut self, tokens: &mut TokenProvider) -> Result<Expression, InterpreterError> {
    match tokens.next() {
      Next::Token(token) => {
        match &token {
          Token::Literal { literal, .. } => match literal {
            Literal::String { .. } | Literal::Number { .. } => return Ok(Expression::Literal { token: token.clone() }),
            _ => {}
          },
          Token::Symbol { symbol, .. } => match symbol {
            Symbol::LeftParenthesis => {
              let operand = Box::new(self.expression(tokens)?);
              let _delimiter = self.pair_delimiter(tokens, Symbol::RightParenthesis)?;
              return Ok(Expression::Grouping { operand });
            }
            Symbol::LeftSquigglyBracket => {
              let operand = Box::new(self.expression(tokens)?);
              let _delimiter = self.pair_delimiter(tokens, Symbol::RightSquigglyBracket)?;
              return Ok(Expression::Grouping { operand });
            }
            _ => {}
          },
          _ => {}
        }

        self.error(InterpreterError::ParseError {
          line: token.line(),
          column: token.column(),
          message: format!("Expected expression but got `{}`", tokens.previous_valid()),
        })
      }
      Next::EndOfFile { .. } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          line: prev.line(),
          column: prev.column() + prev.lexeme().len() as u32,
          message: format!("Expected expression after `{}`", prev),
        })
      }
      Next::EndOfStream { .. } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          line: prev.line(),
          column: prev.column() + prev.lexeme().len() as u32,
          message: format!("Expected expression after `{}`", prev),
        })
      }
    }
  }

  fn pair_delimiter(&mut self, tokens: &mut TokenProvider, delimiter: Symbol) -> Result<Token, InterpreterError> {
    match self.check_delimiter(tokens, &delimiter) {
      Ok(matches) => match tokens.next().cloned() {
        Next::Token(token) => {
          if matches {
            Ok(token)
          } else {
            let delimiter = Token::Symbol {
              line: token.line(),
              column: token.column(),
              symbol: delimiter,
            };
            self.error(InterpreterError::UnmatchedDelimiter {
              line: delimiter.line(),
              column: delimiter.column(),
              delimiter: delimiter.lexeme(),
            })
          }
        }
        Next::EndOfFile { line, column } | Next::EndOfStream { line, column } => {
          let delimiter = Token::Symbol {
            line,
            column,
            symbol: delimiter,
          };
          self.error(InterpreterError::UnmatchedDelimiter {
            line: delimiter.line(),
            column: delimiter.column(),
            delimiter: delimiter.lexeme(),
          })
        }
      },
      Err((line, column)) => {
        let delimiter = Token::Symbol {
          line,
          column,
          symbol: delimiter,
        };
        self.error(InterpreterError::UnmatchedDelimiter {
          line: delimiter.line(),
          column: delimiter.column(),
          delimiter: delimiter.lexeme(),
        })
      }
    }
  }

  fn check_delimiter(&mut self, tokens: &mut TokenProvider, delimiter: &Symbol) -> Result<bool, (u32, u32)> {
    match tokens.peek() {
      Next::Token(Token::Symbol { symbol, .. }) => Ok(symbol == delimiter),
      Next::Token(token) => Err((token.line(), token.column())),
      Next::EndOfFile { line, column } | Next::EndOfStream { line, column } => Err((line, column)),
    }
  }

  fn error<T>(&mut self, error: InterpreterError) -> Result<T, InterpreterError> {
    self.error_handler.get_mut().push(error.clone());

    Err(error)
  }

  #[allow(unused)]
  fn synchronize(&mut self, tokens: &mut TokenProvider) {
    let mut token = tokens.next();
    while let Next::Token(next_token) = token {
      if matches!(next_token, Token::Symbol {
        symbol: Symbol::Semicolon,
        ..
      }) {
        return;
      }

      if matches!(next_token, Token::Keyword {
        keyword: Keyword::If
          | Keyword::For
          | Keyword::While
          | Keyword::Loop
          | Keyword::Return
          | Keyword::Type
          | Keyword::Impl,
        ..
      }) {
        return;
      }

      token = tokens.next();
    }
  }
}
