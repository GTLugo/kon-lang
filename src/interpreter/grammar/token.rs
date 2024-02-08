use std::{
  any::Any,
  fmt::{Display, Formatter},
};

use strum::EnumDiscriminants;

use super::{keyword::Keyword, literal::Literal, symbol::Symbol};
use crate::error::InterpreterError;

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
#[strum_discriminants(enumflags2::bitflags())]
#[strum_discriminants(repr(u16))]
pub enum Token {
  Symbol(SymbolToken),
  Keyword(KeywordToken),
  Literal(LiteralToken),
  EndOfFile { line: u32, column: u32 },
  Invalid { error: InterpreterError },
}

impl Token {
  pub fn value(&self) -> Option<Box<dyn Any>> {
    match self {
      Token::Literal(token) => Some(token.literal.value()),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolToken {
  pub line: u32,
  pub column: u32,
  pub symbol: Symbol,
}

impl Display for SymbolToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.symbol)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeywordToken {
  pub line: u32,
  pub column: u32,
  pub keyword: Keyword,
}

impl Display for KeywordToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.keyword)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralToken {
  pub line: u32,
  pub column: u32,
  pub literal: Literal,
}

impl Display for LiteralToken {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self.literal)
  }
}

impl Token {
  pub fn line(&self) -> u32 {
    match self {
      Token::Symbol(SymbolToken { line, .. }) => *line,
      Token::Keyword(KeywordToken { line, .. }) => *line,
      Token::Literal(LiteralToken { line, .. }) => *line,
      Token::EndOfFile { line, .. } => *line,
      Token::Invalid { .. } => 0,
    }
  }

  pub fn column(&self) -> u32 {
    match self {
      Token::Symbol(SymbolToken { column, .. }) => *column,
      Token::Keyword(KeywordToken { column, .. }) => *column,
      Token::Literal(LiteralToken { column, .. }) => *column,
      Token::EndOfFile { column, .. } => *column,
      Token::Invalid { .. } => 0,
    }
  }

  pub fn lexeme(&self) -> String {
    match self {
      Token::Symbol(SymbolToken { symbol, .. }) => symbol.lexeme(),
      Token::Keyword(KeywordToken { keyword, .. }) => keyword.lexeme(),
      Token::Literal(LiteralToken { literal, .. }) => literal.lexeme(),
      Token::EndOfFile { .. } => "[EOF]".into(),
      Token::Invalid { .. } => "[INV]".into(),
    }
  }

  pub fn reserved_word(value: &str, line: u32, column: u32) -> Option<Self> {
    match value {
      // Keywords
      Keyword::IF => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::If,
      })),
      Keyword::ELSE => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Else,
      })),
      Keyword::FOR => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::For,
      })),
      Keyword::WHILE => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::While,
      })),
      Keyword::LOOP => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Loop,
      })),
      Keyword::RETURN => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Return,
      })),
      Keyword::SELF => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::_Self,
      })),
      Keyword::SUPER => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Super,
      })),
      Keyword::IMPORT => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Import,
      })),
      Keyword::EXPORT => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Export,
      })),
      Keyword::PUBLIC => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Public,
      })),
      Keyword::TYPE => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Type,
      })),
      Keyword::IMPL => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Impl,
      })),
      Keyword::AS => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::As,
      })),
      Keyword::TRAIT => Some(Token::Keyword(KeywordToken {
        line,
        column,
        keyword: Keyword::Trait,
      })),
      _ => None,
    }
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.lexeme())
  }
}
