use crate::interpreter::{
  error::InterpreterError,
  grammar::{symbol::Symbol, token::SymbolToken},
};

#[derive(Debug, PartialEq)]
pub enum BoundUnaryOperator {
  Negation,
  Not,
}

impl TryFrom<SymbolToken> for BoundUnaryOperator {
  type Error = InterpreterError;

  fn try_from(value: SymbolToken) -> Result<Self, Self::Error> {
    match value.symbol {
      Symbol::Minus => Ok(Self::Negation),
      Symbol::ExclamationPoint => Ok(Self::Not),
      _ => Err(InterpreterError::UnknownOperator {
        position: value.position,
        operator: value.symbol.lexeme(),
      }),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum BoundBinaryOperator {
  Addition,
  Subtraction,
  Multiplication,
  Division,
  LessThan,
  GreaterThan,
  LessThanEquals,
  GreaterThanEquals,
  Equals,
  NotEquals,
}

impl TryFrom<SymbolToken> for BoundBinaryOperator {
  type Error = InterpreterError;

  fn try_from(value: SymbolToken) -> Result<Self, Self::Error> {
    match value.symbol {
      Symbol::Plus => Ok(Self::Addition),
      Symbol::Minus => Ok(Self::Subtraction),
      Symbol::Asterisk => Ok(Self::Multiplication),
      Symbol::ForwardSlash => Ok(Self::Division),
      Symbol::LeftAngledBracket => Ok(Self::LessThan),
      Symbol::RightAngledBracket => Ok(Self::GreaterThan),
      Symbol::LeftAngledBracketEquals => Ok(Self::LessThanEquals),
      Symbol::RightAngledBracketEquals => Ok(Self::GreaterThanEquals),
      Symbol::DoubleEquals => Ok(Self::Equals),
      Symbol::ExclamationPointEquals => Ok(Self::NotEquals),
      _ => Err(InterpreterError::UnknownOperator {
        position: value.position,
        operator: value.symbol.lexeme(),
      }),
    }
  }
}
