use std::fmt::{Display, Formatter};

use strum::EnumDiscriminants;

use crate::error::InterpreterError;

use super::{keyword::Keyword, literal::Literal, symbol::Symbol};

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(enumflags2::bitflags())]
#[strum_discriminants(repr(u16))]
pub enum Token {
  Symbol { line: u32, column: u32, symbol: Symbol },
  Keyword { line: u32, column: u32, keyword: Keyword },
  Literal { line: u32, column: u32, literal: Literal },
  EndOfFile { line: u32, column: u32 },
  Invalid { error: InterpreterError },
}

impl Token {
  pub fn line(&self) -> u32 {
    match self {
      Token::Symbol { line, .. } => *line,
      Token::Keyword { line, .. } => *line,
      Token::Literal { line, .. } => *line,
      Token::EndOfFile { line, .. } => *line,
      Token::Invalid { .. } => 0,
    }
  }

  pub fn column(&self) -> u32 {
    match self {
      Token::Symbol { column, .. } => *column,
      Token::Keyword { column, .. } => *column,
      Token::Literal { column, .. } => *column,
      Token::EndOfFile { column, .. } => *column,
      Token::Invalid { .. } => 0,
    }
  }

  pub fn lexeme(&self) -> String {
    match self {
      Token::Symbol { symbol, .. } => symbol.lexeme(),
      Token::Keyword { keyword, .. } => keyword.lexeme(),
      Token::Literal { literal, .. } => literal.lexeme(),
      Token::EndOfFile { .. } => "[EOF]".into(),
      Token::Invalid { .. } => "[INV]".into(),
    }
  }

  pub fn reserved_word(value: &str, line: u32, column: u32) -> Option<Self> {
    match value {
      // Keywords
      Keyword::IF => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::If,
      }),
      Keyword::ELSE => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Else,
      }),
      Keyword::FOR => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::For,
      }),
      Keyword::WHILE => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::While,
      }),
      Keyword::LOOP => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Loop,
      }),
      Keyword::RETURN => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Return,
      }),
      Keyword::SELF => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::_Self,
      }),
      Keyword::SUPER => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Super,
      }),
      Keyword::IMPORT => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Import,
      }),
      Keyword::EXPORT => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Export,
      }),
      Keyword::PUBLIC => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Public,
      }),
      Keyword::TYPE => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Type,
      }),
      Keyword::IMPL => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Impl,
      }),
      Keyword::AS => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::As,
      }),
      Keyword::TRAIT => Some(Token::Keyword {
        line,
        column,
        keyword: Keyword::Trait,
      }),
      _ => None,
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.lexeme())
  }
}