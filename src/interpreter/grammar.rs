use super::token::Token;

pub trait Syntax {}

// pub trait ExpressionSyntax: Syntax {}

pub enum Expression {
    Nothing,
    Literal {
        token: Token,
    },
    Unary {
        operator: Token,
        operand: Box<Expression>,
    },
    Binary {
        operator: Token,
        left_operand: Box<Expression>,
        right_operand: Box<Expression>,
    },
    Grouping {
        operand: Box<Expression>,
    }
}

