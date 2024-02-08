use std::any::{type_name_of_val, Any, TypeId};

use super::bound_operator::{BoundBinaryOperator, BoundUnaryOperator};
use crate::error::InterpreterError;

#[derive(Debug)]
pub enum BoundExpression {
  Literal {
    data_type: TypeId,
    value: Box<dyn Any>,
  },
  Unary {
    data_type: TypeId,
    operator: BoundUnaryOperator,
    operand: Box<BoundExpression>,
  },
  Binary {
    data_type: TypeId,
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
      BoundExpression::Literal { data_type, value } => {
        if *data_type == TypeId::of::<i64>() {
          let Some(&value) = value.downcast_ref::<i64>() else {
            unreachable!()
          };

          return Ok(Box::new(value));
        }

        Err(InterpreterError::SyntaxError {
          line: 0,
          column: 0,
          message: "literals of this value not supported".to_string(),
        })
      }
      BoundExpression::Unary {
        data_type,
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
              line: 0,
              column: 0,
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
              line: 0,
              column: 0,
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
        }
      }
      BoundExpression::Binary {
        data_type,
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
              line: 0,
              column: 0,
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundBinaryOperator::Subtraction => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left - right));
            }

            Err(InterpreterError::SyntaxError {
              line: 0,
              column: 0,
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundBinaryOperator::Multiplication => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left * right));
            }

            Err(InterpreterError::SyntaxError {
              line: 0,
              column: 0,
              message: format!("cannot perform `{:?}` on value", operator),
            })
          }
          BoundBinaryOperator::Division => {
            if let (Some(left), Some(right)) = (left_value.downcast_ref::<i64>(), right_value.downcast_ref::<i64>()) {
              return Ok(Box::new(left / right));
            }

            Err(InterpreterError::SyntaxError {
              line: 0,
              column: 0,
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
