use foxy_utils::types::handle::Handle;

use super::bound_expression::BoundExpression;
use crate::{error::error_handler::ErrorHandler, interpreter::grammar::expression::Expression};

pub struct Binder {
  errors: Handle<ErrorHandler>,
}

impl Binder {
  // pub fn bind(&mut self, syntax: Expression) -> BoundExpression {
  //   match syntax {
  //     Expression::Literal { token } => BoundExpression::Literal { type_, value
  // },     Expression::Unary { operator, operand } => todo!(),
  //     Expression::Binary {
  //       operator,
  //       left_operand,
  //       right_operand,
  //     } => todo!(),
  //     Expression::Grouping { operand } => todo!(),
  //     Expression::Invalid { token } => BoundExpression::Invalid { expression:
  // None },   }
  // }
}
