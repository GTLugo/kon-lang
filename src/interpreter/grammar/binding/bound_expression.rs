use std::any::{Any, TypeId};

use super::bound_operator::{BoundBinaryOperator, BoundUnaryOperator};
use crate::interpreter::{error::InterpreterError, grammar::token::Position};

#[derive(Debug)]
pub enum BoundExpression {
  Literal {
    data_type: TypeId,
    position: Position,
    value: Box<dyn Any>,
  },
  Unary {
    data_type: TypeId,
    position: Position,
    operator: BoundUnaryOperator,
    operand: Box<BoundExpression>,
  },
  Binary {
    data_type: TypeId,
    position: Position,
    operator: BoundBinaryOperator,
    left_operand: Box<BoundExpression>,
    right_operand: Box<BoundExpression>,
  },
  Grouping {
    data_type: TypeId,
    operand: Box<BoundExpression>,
  },
}

impl BoundExpression {
  pub fn data_type(&self) -> TypeId {
    match self {
      BoundExpression::Literal { data_type, .. } => *data_type,
      BoundExpression::Unary { data_type, .. } => *data_type,
      BoundExpression::Binary { data_type, .. } => *data_type,
      BoundExpression::Grouping { data_type, .. } => *data_type,
    }
  }

  pub fn evaluate(&self) -> Result<Box<dyn Any>, InterpreterError> {
    match self {
      BoundExpression::Literal {
        data_type,
        position,
        value,
      } => {
        if let Some(&integer) = value.downcast_ref::<i64>() {
          return Ok(Box::new(integer));
        }

        Err(InterpreterError::SyntaxError {
          position: position.to_owned(),
          message: "literals of this value not supported".to_string(),
        })
      }
      BoundExpression::Unary {
        data_type,
        position,
        operator,
        operand,
      } => {
        let value = operand.evaluate()?;

        match operator {
          BoundUnaryOperator::Negation => {
            if *data_type == TypeId::of::<i64>() {
              let Some(&value) = value.downcast_ref::<i64>() else {
                unreachable!()
              };

              return Ok(Box::new(-value));
            }

            Err(InterpreterError::SyntaxError {
              position: position.to_owned(),
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundUnaryOperator::Not => {
            if *data_type == TypeId::of::<i64>() {
              let Some(&value) = value.downcast_ref::<i64>() else {
                unreachable!()
              };

              return Ok(Box::new(!value));
            }

            Err(InterpreterError::SyntaxError {
              position: position.to_owned(),
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
        }
      }
      BoundExpression::Binary {
        data_type,
        position,
        operator,
        left_operand,
        right_operand,
      } => {
        let left_value = left_operand.evaluate()?;
        let right_value = right_operand.evaluate()?;

        match operator {
          BoundBinaryOperator::Addition => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left + right));
            }

            Err(InterpreterError::SyntaxError {
              position: position.to_owned(),
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundBinaryOperator::Subtraction => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left - right));
            }

            Err(InterpreterError::SyntaxError {
              position: position.to_owned(),
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundBinaryOperator::Multiplication => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left * right));
            }

            Err(InterpreterError::SyntaxError {
              position: position.to_owned(),
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundBinaryOperator::Division => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left / right));
            }

            Err(InterpreterError::SyntaxError {
              position: position.to_owned(),
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          _ => Err(InterpreterError::Other("expected binary expression".to_string())),
        }
      }
      BoundExpression::Grouping { operand, .. } => operand.evaluate(),
    }
  }
}
