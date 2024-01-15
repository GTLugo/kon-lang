use std::fmt::Debug;

use super::token::Token;

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
    pub fn evaluate(&self) {
        match self {
            Expression::Literal { token } => {
                println!("literal: {token}");
            }
            Expression::Unary { operator, operand } => {
                let value = operand.evaluate();
                println!("unary: {operator}");
            }
            Expression::Binary {
                operator,
                left_operand,
                right_operand,
            } => {
                let left_value = left_operand.evaluate();
                let right_value = right_operand.evaluate();
                println!("binary: {operator}");
            }
            Expression::Grouping { operand } => {
                let value = operand.evaluate();
                println!("grouping");
            }
            Expression::Invalid => {
                println!("invalid expression");
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
