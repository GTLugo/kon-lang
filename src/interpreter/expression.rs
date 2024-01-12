use super::token::Token;

pub trait Syntax {}

pub trait ExpressionSyntax: Syntax {}

pub enum Expression {
    Nothing,
    Number {
        token: Token,
    },
    Unary {
        operator: Token,
        operand: Box<dyn ExpressionSyntax>,
    },
    Binary {
        operator: Token,
        left_operand: Box<dyn ExpressionSyntax>,
        right_operand: Box<dyn ExpressionSyntax>,
    },
}

impl Expression {
    pub fn evaluate(&self) -> Expression {
        todo!()
    }
}
