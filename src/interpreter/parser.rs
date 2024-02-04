use enumflags2::BitFlags;
use foxy_utils::types::handle::Handle;

use super::{
  grammar::{Expression, SyntaxTree},
  token::{Keyword, Literal, Symbol, Token, TokenDiscriminants},
  token_provider::{Next, TokenProvider},
};
use crate::error::{error_handler::ErrorHandler, InterpreterError};

#[derive(Debug, PartialEq, Eq)]
enum DelimiterType {
  Paren,
  Curly,
  Angled,
  Square,
}

#[derive(Debug, Eq)]
struct Delimiter {
  delimiter: DelimiterType,
  line: u32,
  column: u32,
}

impl Delimiter {
  pub fn lexeme(&self) -> String {
    match self.delimiter {
      DelimiterType::Paren => "(".into(),
      DelimiterType::Curly => "{".into(),
      DelimiterType::Angled => "<".into(),
      DelimiterType::Square => "[".into(),
    }
  }

  fn line(&self) -> u32 {
    self.line
  }

  fn column(&self) -> u32 {
    self.column
  }
}

impl PartialEq for Delimiter {
  fn eq(&self, other: &Self) -> bool {
    self.delimiter == other.delimiter
  }
}

impl TryFrom<Token> for Delimiter {
  type Error = ();

  fn try_from(token: Token) -> Result<Self, Self::Error> {
    let Token::Symbol { symbol, .. } = token else {
      return Err(());
    };

    if !matches!(
      symbol,
      Symbol::RightParenthesis | Symbol::RightAngledBracket | Symbol::RightCurlyBracket | Symbol::RightSquareBracket
    ) {
      return Err(());
    }

    let delimiter = match symbol {
      Symbol::LeftParenthesis => DelimiterType::Paren,
      Symbol::LeftAngledBracket => DelimiterType::Angled,
      Symbol::LeftCurlyBracket => DelimiterType::Curly,
      Symbol::LeftSquareBracket => DelimiterType::Square,
      Symbol::RightParenthesis => DelimiterType::Paren,
      Symbol::RightAngledBracket => DelimiterType::Angled,
      Symbol::RightCurlyBracket => DelimiterType::Curly,
      Symbol::RightSquareBracket => DelimiterType::Square,
      _ => return Err(()),
    };

    Ok(Self {
      delimiter,
      line: token.line(),
      column: token.column(),
    })
  }
}

pub struct Parser {
  error_handler: Handle<ErrorHandler>,
  delimiter_stack: Vec<Delimiter>,
}

impl Parser {
  pub fn new(error_handler: Handle<ErrorHandler>) -> Self {
    Self {
      error_handler,
      delimiter_stack: Default::default(),
    }
  }

  pub fn parse(&mut self, tokens: &[Token]) -> SyntaxTree {
    let mut tokens = TokenProvider::new(tokens);
    let root = self.expression(&mut tokens);

    let next = tokens.peek().cloned();
    match next {
      Next::Token(t) => {
        self.error_handler.get_mut().push(InterpreterError::ParseError {
          line: t.line(),
          column: t.column(),
          message: format!("Expected EOF but got `{}`", t),
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

  fn is_rogue_delimiter(&mut self, token: Next<&Token>) -> bool {
    let Next::Token(token) = token.cloned() else {
      return false;
    };

    let Ok(delim) = token.try_into() else {
      return false;
    };

    !self.delimiter_stack.ends_with(&[delim])
  }

  fn match_token(&mut self, tokens: &mut TokenProvider, types: BitFlags<TokenDiscriminants>) -> Option<Token> {
    let mut peeked = tokens.peek();

    if self.is_rogue_delimiter(peeked.clone()) {
      let Next::Token(rogue) = peeked else {
        return None;
      };
      self.error(InterpreterError::ParseError {
        line: rogue.line(),
        column: rogue.column(),
        message: format!("Rogue `{}`", rogue),
      });
      tokens.next(); // consume the delimiter
      peeked = tokens.peek();
    }

    let Next::Token(peeked) = peeked else {
      return None;
    };

    let discrm = TokenDiscriminants::from(peeked);

    if types.contains(discrm) {
      Some(peeked.clone())
    } else {
      None
    }
  }

  fn match_symbol(&self, token: &Token, types: BitFlags<Symbol>) -> bool {
    let Token::Symbol { symbol, .. } = token else {
      return false;
    };

    types.contains(*symbol)
  }

  fn expression(&mut self, tokens: &mut TokenProvider) -> Expression {
    self.equality(tokens)
  }

  fn equality(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.comparison(tokens);

    while {
      if let Some(token) = self.match_token(tokens, TokenDiscriminants::Symbol.into()) {
        self.match_symbol(&token, Symbol::DoubleEquals | Symbol::ExclamationPointEquals)
      } else {
        false
      }
    } {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.comparison(tokens));
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    expression
  }

  fn comparison(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.term(tokens);

    while if let Some(token) = self.match_token(tokens, TokenDiscriminants::Symbol.into()) {
      self.match_symbol(
        &token,
        Symbol::LeftAngledBracket
          | Symbol::RightAngledBracket
          | Symbol::LeftAngledBracketEquals
          | Symbol::RightAngledBracketEquals,
      )
    } else {
      false
    } {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.term(tokens));
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    expression
  }

  fn term(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.factor(tokens);

    while if let Some(token) = self.match_token(tokens, TokenDiscriminants::Symbol.into()) {
      self.match_symbol(&token, Symbol::Plus | Symbol::Minus)
    } else {
      false
    } {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.factor(tokens));
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    expression
  }

  fn factor(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.power(tokens);

    while if let Some(token) = self.match_token(tokens, TokenDiscriminants::Symbol.into()) {
      self.match_symbol(&token, Symbol::Asterisk | Symbol::ForwardSlash)
    } else {
      false
    } {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.power(tokens));
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    expression
  }

  fn power(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.unary(tokens);

    while if let Some(token) = self.match_token(tokens, TokenDiscriminants::Symbol.into()) {
      self.match_symbol(&token, Symbol::Caret.into())
    } else {
      false
    } {
      if let Next::Token(operator) = tokens.next().cloned() {
        let right_operand = Box::new(self.unary(tokens));
        expression = Expression::Binary {
          left_operand: Box::new(expression),
          operator,
          right_operand,
        };
      }
    }

    expression
  }

  fn unary(&mut self, tokens: &mut TokenProvider) -> Expression {
    if if let Some(token) = self.match_token(tokens, TokenDiscriminants::Symbol.into()) {
      self.match_symbol(&token, Symbol::ExclamationPoint | Symbol::Minus)
    } else {
      false
    } {
      if let Next::Token(operator) = tokens.next().cloned() {
        let operand = Box::new(self.unary(tokens));
        return Expression::Unary { operator, operand };
      }
    }

    self.primary(tokens)
  }

  fn primary(&mut self, tokens: &mut TokenProvider) -> Expression {
    let next_token = tokens.next().cloned();
    match next_token {
      Next::Token(token) => {
        match &token {
          Token::Literal { literal, .. } => match literal {
            Literal::String { .. } | Literal::Number { .. } => return Expression::Literal { token: token.clone() },
            _ => {}
          },
          Token::Symbol { line, column, symbol } => match symbol {
            Symbol::LeftParenthesis => {
              self.delimiter_stack.push(Delimiter {
                delimiter: DelimiterType::Paren,
                line: *line,
                column: *column,
              });
              let operand = Box::new(self.expression(tokens));
              let _delimiter = self.pair_delimiter(tokens, Symbol::RightParenthesis);
              return Expression::Grouping { operand };
            }
            Symbol::LeftCurlyBracket => {
              self.delimiter_stack.push(Delimiter {
                delimiter: DelimiterType::Curly,
                line: *line,
                column: *column,
              });
              let operand = Box::new(self.expression(tokens));
              let _delimiter = self.pair_delimiter(tokens, Symbol::RightCurlyBracket);
              self.delimiter_stack.pop();
              return Expression::Grouping { operand };
            }
            _ => {}
          },
          _ => {}
        }

        self.error(InterpreterError::ParseError {
          line: token.line(),
          column: token.column(),
          message: format!("Expected expression but got `{}`", tokens.previous_valid()),
        });

        Expression::Invalid {
          token: Some(Token::Keyword {
            line: token.line(),
            column: token.column(),
            keyword: Keyword::Void,
          }),
        }
      }
      Next::EndOfFile { line, column } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          line: prev.line(),
          column: prev.column() + prev.lexeme().len() as u32,
          message: format!("Expected expression after `{}`", prev),
        });
        Expression::Invalid {
          token: Some(Token::Keyword {
            line,
            column,
            keyword: Keyword::Void,
          }),
        }
      }
      Next::EndOfStream { line, column } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          line: prev.line(),
          column: prev.column() + prev.lexeme().len() as u32,
          message: format!("Expected expression after `{}`", prev),
        });
        Expression::Invalid {
          token: Some(Token::Keyword {
            line,
            column,
            keyword: Keyword::Void,
          }),
        }
      }
    }
  }

  fn pair_delimiter(&mut self, tokens: &mut TokenProvider, delimiter: Symbol) -> Token {
    let unmatched = self.delimiter_stack.pop().expect("expected Some(delimiter)");
    match self.check_delimiter(tokens, &delimiter) {
      Ok(matches) => match tokens.next().cloned() {
        Next::Token(token) => {
          if matches {
            token
          } else {
            let delimiter = Token::Symbol {
              line: token.line(),
              column: token.column(),
              symbol: delimiter,
            };
            self.error(InterpreterError::UnmatchedDelimiter {
              line: unmatched.line(),
              column: unmatched.column(),
              delimiter: unmatched.lexeme(),
            });
            delimiter
          }
        }
        Next::EndOfFile { line, column } | Next::EndOfStream { line, column } => {
          let delimiter = Token::Symbol {
            line,
            column,
            symbol: delimiter,
          };
          self.error(InterpreterError::UnmatchedDelimiter {
            line: unmatched.line(),
            column: unmatched.column(),
            delimiter: unmatched.lexeme(),
          });
          delimiter
        }
      },
      Err((line, column)) => {
        let delimiter = Token::Symbol {
          line,
          column,
          symbol: delimiter,
        };
        self.error(InterpreterError::UnmatchedDelimiter {
          line: unmatched.line(),
          column: unmatched.column(),
          delimiter: unmatched.lexeme(),
        });
        delimiter
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

  fn error(&mut self, error: InterpreterError) {
    self.error_handler.get_mut().push(error.clone());
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
