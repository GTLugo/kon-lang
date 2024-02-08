#[derive(Debug, PartialEq)]
pub enum BoundUnaryOperator {
  Negation,
}

#[derive(Debug, PartialEq)]
pub enum BoundBinaryOperator {
  Addition,
  Subtraction,
  Multiplication,
  Division,
  Exponential,
  LessThan,
  GreaterThan,
  LessThanEquals,
  GreaterThanEquals,
  Equals,
  NotEquals,
}
