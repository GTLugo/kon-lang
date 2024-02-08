use foxy_utils::types::handle::Handle;

use super::{
  bound_expression::BoundExpression,
  bound_operator::{BoundBinaryOperator, BoundUnaryOperator},
};
use crate::{error::error_handler::ErrorHandler, interpreter::grammar::expression::Expression};

pub struct Binder {
  errors: Handle<ErrorHandler>,
}

impl Binder {
  pub fn new(errors: Handle<ErrorHandler>) -> Self {
    Self { errors }
  }

  pub fn bind(&mut self, syntax: Expression) -> BoundExpression {
    match syntax {
      Expression::Literal { token } => {
        let value = token.literal.value();
        BoundExpression::Literal {
          data_type: (*value).type_id(),
          value,
        }
      }
      Expression::Unary { operator, operand } => {
        let operand = Box::new(self.bind(*operand));
        let operator = match BoundUnaryOperator::try_from(operator) {
          Ok(value) => value,
          Err(error) => {
            self.errors.get_mut().push(error);
            return *operand;
          }
        };
        BoundExpression::Unary {
          data_type: operand.data_type(),
          operator,
          operand,
        }
      }
      Expression::Binary {
        operator,
        left_operand,
        right_operand,
      } => {
        let left_operand = Box::new(self.bind(*left_operand));
        let right_operand = Box::new(self.bind(*right_operand));
        let operator = match BoundBinaryOperator::try_from(operator) {
          Ok(value) => value,
          Err(error) => {
            self.errors.get_mut().push(error);
            return *left_operand;
          }
        };
        BoundExpression::Binary {
          data_type: left_operand.data_type(),
          operator,
          left_operand,
          right_operand,
        }
      }
      Expression::Grouping { operand } => {
        let operand = Box::new(self.bind(*operand));
        BoundExpression::Grouping {
          data_type: operand.data_type(),
          operand,
        }
      }
    }
  }
}
