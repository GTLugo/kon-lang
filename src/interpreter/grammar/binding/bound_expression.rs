use std::any::{Any, TypeId};

use super::bound_operator::{BoundBinaryOperator, BoundUnaryOperator};

#[derive(Debug)]
pub enum BoundExpression {
  Invalid {
    expression: Option<Box<BoundExpression>>,
  },
  Literal {
    type_: TypeId,
    value: Option<Box<dyn Any>>,
  },
  Unary {
    operator: BoundUnaryOperator,
    operand: Box<BoundExpression>,
  },
  Binary {
    operator: BoundBinaryOperator,
    left_operand: Box<BoundExpression>,
    right_operand: Box<BoundExpression>,
  },
  Grouping {
    operand: Box<BoundExpression>,
  },
}
