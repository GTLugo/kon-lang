use enumflags2::BitFlags;
use foxy_utils::types::handle::Handle;
use tracing::debug;

use super::{
  grammar::{
    expression::Expression,
    keyword::Keyword,
    literal::Literal,
    symbol::Symbol,
    syntax_tree::SyntaxTree,
    token::{KeywordToken, LiteralToken, SymbolToken, Token, TokenDiscriminants},
  },
  util::token_provider::{Next, TokenProvider},
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
    let Token::Symbol(SymbolToken { symbol, .. }) = token else {
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

  fn match_token_types(&mut self, tokens: &mut TokenProvider, types: BitFlags<TokenDiscriminants>) -> Option<Token> {
    let mut peeked = tokens.peek();
    // check for rogue delimiters
    if self.is_rogue_delimiter(peeked.clone()) {
      let Next::Token(&Token::Symbol(SymbolToken { line, column, symbol })) = peeked else {
        return None;
      };
      self.error(InterpreterError::UnmatchedDelimiter {
        line,
        column,
        delimiter: symbol.lexeme(),
      });
      tokens.next(); // consume the delimiter
      peeked = tokens.peek();
    }

    // check if no tokens
    let Next::Token(peeked) = peeked else {
      return None;
    };

    // check if matches params
    let discrm = TokenDiscriminants::from(peeked);
    if types.contains(discrm) {
      Some(peeked.clone())
    } else {
      None
    }
  }

  fn match_symbols(&mut self, tokens: &mut TokenProvider, types: BitFlags<Symbol>) -> Option<SymbolToken> {
    if let Some(Token::Symbol(symbol_token)) = self.match_token_types(tokens, TokenDiscriminants::Symbol.into()) {
      if types.contains(symbol_token.symbol) {
        if let Next::Token(Token::Symbol(symbol_token)) = tokens.next() {
          return Some(symbol_token.clone());
        }
      }
    }

    None
  }

  fn expression(&mut self, tokens: &mut TokenProvider) -> Expression {
    self.equality(tokens)
  }

  fn equality(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.comparison(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::DoubleEquals | Symbol::ExclamationPointEquals) {
      let right_operand = Box::new(self.comparison(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn comparison(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.term(tokens);

    while let Some(operator) = self.match_symbols(
      tokens,
      Symbol::LeftAngledBracket
        | Symbol::RightAngledBracket
        | Symbol::LeftAngledBracketEquals
        | Symbol::RightAngledBracketEquals,
    ) {
      let right_operand = Box::new(self.term(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn term(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.factor(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::Plus | Symbol::Minus) {
      let right_operand = Box::new(self.factor(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  fn factor(&mut self, tokens: &mut TokenProvider) -> Expression {
    let mut expression = self.unary(tokens);

    while let Some(operator) = self.match_symbols(tokens, Symbol::Asterisk | Symbol::ForwardSlash) {
      let right_operand = Box::new(self.unary(tokens));
      expression = Expression::Binary {
        left_operand: Box::new(expression),
        operator,
        right_operand,
      };
    }

    expression
  }

  // fn power(&mut self, tokens: &mut TokenProvider) -> Expression {
  //   let mut expression = self.unary(tokens);

  //   while let Some(operator) = self.match_symbols(tokens, Symbol::Caret.into())
  // {     let right_operand = Box::new(self.unary(tokens));
  //     expression = Expression::Binary {
  //       left_operand: Box::new(expression),
  //       operator,
  //       right_operand,
  //     };
  //   }

  //   expression
  // }

  fn unary(&mut self, tokens: &mut TokenProvider) -> Expression {
    if let Some(operator) = self.match_symbols(tokens, Symbol::ExclamationPoint | Symbol::Minus) {
      let operand = Box::new(self.unary(tokens));
      return Expression::Unary { operator, operand };
    }

    self.primary(tokens)
  }

  fn primary(&mut self, tokens: &mut TokenProvider) -> Expression {
    let next_token = tokens.next().cloned();
    match next_token {
      Next::Token(token) => {
        match &token {
          Token::Literal(token) => return Expression::Literal { token: token.clone() },
          Token::Symbol(SymbolToken { line, column, symbol }) => match symbol {
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

        Expression::Literal {
          token: LiteralToken {
            line: token.line(),
            column: token.column(),
            literal: Literal::Void,
          },
        }
      }
      Next::EndOfFile { line, column } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          line: prev.line(),
          column: prev.column() + prev.lexeme().len() as u32,
          message: format!("Expected expression after `{}`", prev),
        });
        
        Expression::Literal {
          token: LiteralToken {
            line,
            column,
            literal: Literal::Void,
          },
        }
      }
      Next::EndOfStream { line, column } => {
        let prev = tokens.previous_valid();
        self.error(InterpreterError::ParseError {
          line: prev.line(),
          column: prev.column() + prev.lexeme().len() as u32,
          message: format!("Expected expression after `{}`", prev),
        });
        
        Expression::Literal {
          token: LiteralToken {
            line,
            column,
            literal: Literal::Void,
          },
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
            let delimiter = Token::Symbol(SymbolToken {
              line: token.line(),
              column: token.column(),
              symbol: delimiter,
            });
            self.error(InterpreterError::UnmatchedDelimiter {
              line: unmatched.line(),
              column: unmatched.column(),
              delimiter: unmatched.lexeme(),
            });
            delimiter
          }
        }
        Next::EndOfFile { line, column } | Next::EndOfStream { line, column } => {
          let delimiter = Token::Symbol(SymbolToken {
            line,
            column,
            symbol: delimiter,
          });
          self.error(InterpreterError::UnmatchedDelimiter {
            line: unmatched.line(),
            column: unmatched.column(),
            delimiter: unmatched.lexeme(),
          });
          delimiter
        }
      },
      Err((line, column)) => {
        let delimiter = Token::Symbol(SymbolToken {
          line,
          column,
          symbol: delimiter,
        });
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
      Next::Token(Token::Symbol(symbol_token)) => Ok(symbol_token.symbol == *delimiter),
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
      if matches!(
        next_token,
        Token::Symbol(SymbolToken {
          symbol: Symbol::Semicolon,
          ..
        })
      ) {
        return;
      }

      if matches!(
        next_token,
        Token::Keyword(KeywordToken {
          keyword: Keyword::If
            | Keyword::For
            | Keyword::While
            | Keyword::Loop
            | Keyword::Return
            | Keyword::Type
            | Keyword::Impl,
          ..
        })
      ) {
        return;
      }

      token = tokens.next();
    }
  }
}
