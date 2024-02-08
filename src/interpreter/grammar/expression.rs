use std::{
  any::{Any, TypeId},
  fmt::Display,
};

use super::{
  literal::Literal,
  symbol::Symbol,
  token::{LiteralToken, SymbolToken, Token},
};
use crate::error::InterpreterError;

#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal {
    token: LiteralToken,
  },
  Unary {
    operator: SymbolToken,
    operand: Box<Expression>,
  },
  Binary {
    operator: SymbolToken,
    left_operand: Box<Expression>,
    right_operand: Box<Expression>,
  },
  Grouping {
    operand: Box<Expression>,
  },
}

impl Expression {
  // pub fn evaluate(&self) -> Result<Box<dyn Any>, InterpreterError> {
  //   match self {
  //     Expression::Literal { token } => match token.literal.clone() {
  //       Literal::Identifier { lexeme } => Ok(Box::new(lexeme)),
  //       Literal::String { lexeme } => Ok(Box::new(lexeme)),
  //       Literal::Number { lexeme } => Ok(Box::new(lexeme)),
  //       Literal::Void => Ok(Box::new(())),
  //     },
  //     Expression::Unary { operator, operand } => {
  //       let value = operand.evaluate()?;

  //       if value.type_id() == TypeId::of::<i64>() {
  //         let Some(&value) = value.downcast_ref::<i64>() else {
  //           unreachable!()
  //         };

  //         return match operator.symbol {
  //           Symbol::Minus => Ok(Box::new(-value)),
  //           _ => Err(InterpreterError::SyntaxError {
  //             line: operator.line,
  //             column: operator.column,
  //             message: format!("cannot perform `{}` on i64", operator.symbol.lexeme()),
  //           }),
  //         };
  //       } else if value.type_id() == TypeId::of::<String>() {
  //         return Err(InterpreterError::SyntaxError {
  //           line: operator.line,
  //           column: operator.column,
  //           message: format!("cannot perform `{}` on string", operator.symbol.lexeme()),
  //         });
  //       }

  //       Err(InterpreterError::Other("expected unary expression".to_string()))
  //     }
  //     Expression::Binary {
  //       operator,
  //       left_operand,
  //       right_operand,
  //     } => {
  //       let left_value = left_operand.evaluate()?;
  //       let right_value = right_operand.evaluate()?;

  //       if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
  //         return match operator.symbol {
  //           Symbol::Plus => Ok(Box::new(left + right)),
  //           Symbol::Minus => Ok(Box::new(left - right)),
  //           Symbol::Asterisk => Ok(Box::new(left * right)),
  //           Symbol::ForwardSlash => Ok(Box::new(left / right)),
  //           _ => Err(InterpreterError::SyntaxError {
  //             line: operator.line,
  //             column: operator.column,
  //             message: format!("cannot perform `{}` on i64", operator.symbol.lexeme()),
  //           }),
  //         };
  //       }

  //       if let (Some(left), Some(right)) = (left_value.downcast_ref::<String>(), right_value.downcast_ref::<String>()) {
  //         return match operator.symbol {
  //           Symbol::Plus => Ok(Box::new(format!("{left}{right}"))),
  //           _ => Err(InterpreterError::SyntaxError {
  //             line: operator.line,
  //             column: operator.column,
  //             message: format!("cannot perform `{}` on i64", operator.symbol.lexeme()),
  //           }),
  //         };
  //       }

  //       Err(InterpreterError::Other("expected binary expression".to_string()))
  //     }
  //     Expression::Grouping { operand } => operand.evaluate(),
  //   }
  // }

  pub fn pretty_print(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const INCREMENT: usize = 2;
    write!(f, "{:indent$}", "")?;
    match self {
      Expression::Literal { token } => {
        writeln!(f, "Literal: {token}")?;
      }
      Expression::Unary { operator, operand } => {
        writeln!(f, "Unary: {operator}")?;
        operand.pretty_print(indent + INCREMENT, f)?;
      }
      Expression::Binary {
        operator,
        left_operand,
        right_operand,
      } => {
        writeln!(f, "Binary: {operator}")?;
        left_operand.pretty_print(indent + INCREMENT, f)?;
        right_operand.pretty_print(indent + INCREMENT, f)?;
      }
      Expression::Grouping { operand } => {
        writeln!(f, "Grouping")?;
        operand.pretty_print(indent + INCREMENT, f)?;
      }
    }

    Ok(())
  }
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.pretty_print(0, f)
  }
}
