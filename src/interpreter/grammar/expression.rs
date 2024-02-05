use std::{any::Any, fmt::Display};

use crate::error::InterpreterError;

use super::{literal::Literal, symbol::Symbol, token::Token};

#[derive(Debug, PartialEq)]
pub enum Expression {
  Literal {
    token: Token,
  },
  Unary {
    operator: Token,
    operand: Box<Expression>,
  },
  Binary {
    operator: Token,
    left_operand: Box<Expression>,
    right_operand: Box<Expression>,
  },
  Grouping {
    operand: Box<Expression>,
  },
  Invalid {
    token: Option<Token>,
  },
}

impl Expression {
  pub fn evaluate(&self) -> Result<Box<dyn Any>, InterpreterError> {
    match self {
      Expression::Literal { token } => match token {
        Token::Literal { literal, .. } => match literal.clone() {
          Literal::Identifier { lexeme } => Ok(Box::new(lexeme)),
          Literal::String { lexeme } => Ok(Box::new(lexeme)),
          Literal::Number { lexeme } => Ok(Box::new(lexeme)),
          Literal::Void => Ok(Box::new(())),
        },
        _ => unreachable!("only literals should enter this branch"),
      },
      Expression::Unary { operator, operand } => {
        let operator_symbol = match operator {
          Token::Symbol { symbol, .. } => symbol,
          _ => unreachable!("only symbols should enter this branch"),
        };

        let value = operand.evaluate()?;

        if let Some(&value) = value.downcast_ref::<f64>() {
          return match operator_symbol {
            Symbol::Minus => Ok(Box::new(-value)),
            _ => Err(InterpreterError::SyntaxError {
              line: operator.line(),
              column: operator.column(),
              message: format!("cannot perform `{}` on f64", operator_symbol.lexeme()),
            }),
          };
        }

        if value.downcast_ref::<String>().is_some() {
          return Err(InterpreterError::SyntaxError {
            line: operator.line(),
            column: operator.column(),
            message: format!("cannot perform `{}` on string", operator_symbol.lexeme()),
          });
        }

        Err(InterpreterError::Unspecified)
      }
      Expression::Binary {
        operator,
        left_operand,
        right_operand,
      } => {
        let operator_symbol = match operator {
          Token::Symbol { symbol, .. } => symbol,
          _ => unreachable!("only symbols should enter this branch"),
        };

        let left_value = left_operand.evaluate()?;
        let right_value = right_operand.evaluate()?;

        if let (Some(left), Some(right)) = (left_value.downcast_ref::<f64>(), right_value.downcast_ref::<f64>()) {
          return match operator_symbol {
            Symbol::Plus => Ok(Box::new(left + right)),
            Symbol::Minus => Ok(Box::new(left - right)),
            Symbol::Asterisk => Ok(Box::new(left * right)),
            Symbol::ForwardSlash => Ok(Box::new(left / right)),
            Symbol::Caret => Ok(Box::new(left.powf(*right))),
            _ => Err(InterpreterError::SyntaxError {
              line: operator.line(),
              column: operator.column(),
              message: format!("cannot perform `{}` on f64", operator_symbol.lexeme()),
            }),
          };
        }

        if let (Some(left), Some(right)) = (left_value.downcast_ref::<String>(), right_value.downcast_ref::<String>()) {
          return match operator_symbol {
            Symbol::Plus => Ok(Box::new(format!("{left}{right}"))),
            _ => Err(InterpreterError::SyntaxError {
              line: operator.line(),
              column: operator.column(),
              message: format!("cannot perform `{}` on f64", operator_symbol.lexeme()),
            }),
          };
        }

        Err(InterpreterError::Unspecified)
      }
      Expression::Grouping { operand } => operand.evaluate(),
      Expression::Invalid { .. } => Expression::Literal {
        token: Token::Literal {
          line: 0,
          column: 0,
          literal: Literal::Void,
        },
      }
      .evaluate(),
    }
  }

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
      Expression::Invalid { token } => writeln!(f, "[Invalid] token?: {token:?}")?,
    }

    Ok(())
  }
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.pretty_print(0, f)
  }
}