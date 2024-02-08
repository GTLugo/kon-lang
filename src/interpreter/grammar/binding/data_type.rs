use std::any::TypeId;

#[derive(Debug, PartialEq)]
pub enum Type {
  Primitive(Primitive),
  UserDefined(TypeId),
}

#[derive(Debug, PartialEq)]
pub enum Primitive {
  Float,
  Int,
}
