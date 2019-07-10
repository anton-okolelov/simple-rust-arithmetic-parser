use std::fmt;

use Expression::*;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    UnaryMinus(Box<Expression>),
    Value(i64),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value(value) => write!(f, "{}", value),
            Add(left, right) => write!(f, "({}+{})", left, right),
            Subtract(left, right) => write!(f, "({}-{})", left, right),
            UnaryMinus(operand) => write!(f, "-{}", operand),
            Multiply(left, right) => write!(f, "({}*{})", left, right),
            Divide(left, right) => write!(f, "({}/{})", left, right),
        }
    }
}
