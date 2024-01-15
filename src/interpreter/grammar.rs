use std::any::Any;

use crate::error::InterpreterError;

use super::token::{Token, Literal, Symbol};

#[derive(Debug)]
pub enum Expression {
    Literal {
        token: Token,
    },
    Unary {
        operator: Token,
        operand: Box<Expression>,
    },
    Binary {
        left_operand: Box<Expression>,
        operator: Token,
        right_operand: Box<Expression>,
    },
    Grouping {
        operand: Box<Expression>,
    },
    Invalid,
}

impl Expression {
    pub fn evaluate(&self) -> Result<Box<dyn Any>, InterpreterError> {
        match self {
            Expression::Literal { token } => {
                match token {
                    Token::Literal { literal, .. } => match literal.clone() {
                        Literal::Void => Ok(Box::new(())),
                        Literal::Identifier { lexeme } => Ok(Box::new(lexeme)),
                        Literal::String { lexeme } => Ok(Box::new(lexeme)),
                        Literal::Number { lexeme } => Ok(Box::new(lexeme)),
                    }
                    _ => unreachable!("only literals should enter this branch"),
                }
            }
            Expression::Unary { operator, operand } => {
                let operator_symbol = match operator {
                    Token::Symbol {  symbol, .. } => symbol.clone(),
                    _ => unreachable!("only symbols should enter this branch"),
                };

                let value = operand.evaluate()?;

                if let Some(&value) = value.downcast_ref::<f64>() {
                    return match operator_symbol {
                        Symbol::Minus => Ok(Box::new(-value)),
                        _ => Err(InterpreterError::SyntaxError { 
                            line: operator.line(), 
                            column: operator.column(), 
                            message: format!("cannot perform `{}` on f64", operator_symbol.lexeme())
                        }),
                    }
                }

                if value.downcast_ref::<String>().is_some() {
                    return Err(InterpreterError::SyntaxError { 
                        line: operator.line(), 
                        column: operator.column(), 
                        message: format!("cannot perform `{}` on string", operator_symbol.lexeme())
                    })
                }

                Err(InterpreterError::Unspecified)
            }
            Expression::Binary {
                operator,
                left_operand,
                right_operand,
            } => {
                let operator_symbol = match operator {
                    Token::Symbol { symbol, .. } => symbol.clone(),
                    _ => unreachable!("only symbols should enter this branch"),
                };

                let left_value = left_operand.evaluate()?;
                let right_value = right_operand.evaluate()?;

                if let (Some(left), Some(right)) = (left_value.downcast_ref::<f64>(), right_value.downcast_ref::<f64>()) {
                    return match operator_symbol {
                        Symbol::Plus => Ok(Box::new(left + right)),
                        Symbol::Minus => Ok(Box::new(left - right)),
                        Symbol::Asterisk => Ok(Box::new(left * right)),
                        Symbol::ForwardSlash => Ok(Box::new(left + right)),
                        _ => Err(InterpreterError::SyntaxError { 
                            line: operator.line(), 
                            column: operator.column(), 
                            message: format!("cannot perform `{}` on f64", operator_symbol.lexeme())
                        }),
                    }
                }

                if let (Some(left), Some(right)) = (left_value.downcast_ref::<String>(), right_value.downcast_ref::<String>()) {
                    return match operator_symbol {
                        Symbol::Plus => Ok(Box::new(format!("{left}{right}"))),
                        _ => Err(InterpreterError::SyntaxError { 
                            line: operator.line(), 
                            column: operator.column(), 
                            message: format!("cannot perform `{}` on f64", operator_symbol.lexeme())
                        }),
                    }
                }

                Err(InterpreterError::Unspecified)
            }
            Expression::Grouping { operand } => {
                operand.evaluate()
            }
            Expression::Invalid => {
                Err(InterpreterError::Unspecified)
            }
        }
    }
}

// #[derive(Debug)]
// pub enum GrammarRule {
//     Expression(Expression),
//     Equality(Equality),
//     Comparison(Comparison),
//     Term(Term),
//     Factor(Factor),
//     Unary(Unary),
//     Primary(Primary),
//     Void,
// }

// impl GrammarRule {
//     pub fn evaluate(&self) {
//         match self {
//             GrammarRule::Expression(_) => todo!(),
//             GrammarRule::Equality(_) => todo!(),
//             GrammarRule::Comparison(_) => todo!(),
//             GrammarRule::Term(_) => todo!(),
//             GrammarRule::Factor(_) => todo!(),
//             GrammarRule::Unary(_) => todo!(),
//             GrammarRule::Primary(_) => todo!(),
//             GrammarRule::Void => todo!(),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum Expression {
//     Equality(Equality),
// }

// #[derive(Debug)]
// pub enum Equality {
//     Comparison(Comparison),
//     Equality {
//         left_operand: Box<Equality>,
//         operator: Token,
//         right_operand: Comparison,
//     },
// }

// #[derive(Debug)]
// pub enum Comparison {
//     Term(Term),
//     Comparison {
//         left_operand: Box<Comparison>,
//         operator: Token,
//         right_operand: Term,
//     },
// }

// #[derive(Debug)]
// pub enum Term {
//     Factor(Factor),
//     Term {
//         left_operand: Box<Term>,
//         operator: Token,
//         right_operand: Factor,
//     },
// }

// #[derive(Debug)]
// pub enum Factor {
//     Unary(Unary),
//     Factor {
//         left_operand: Box<Factor>,
//         operator: Token,
//         right_operand: Unary,
//     },
// }

// #[derive(Debug)]
// pub enum Unary {
//     Primary(Primary),
//     Unary {
//         operator: Token,
//         operand: Box<Primary>,
//     }
// }

// #[derive(Debug)]
// pub enum Primary {
//     Grouping {
//         operand: Box<Expression>,
//     },
//     Literal {
//         token: Token,
//     },
// }
