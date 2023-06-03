use crate::scanner::{Token, Value};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    }, //assignment
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    }, //binary operations (+, -, .., ==, !=, <=, ..)
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Grouping(Box<Expr>), // "(" expression ")"
    Literal(Value),

    Unary {
        operator: Token,
        expression: Box<Expr>,
    }, // ! or - (negate)
    Variable(Token),
    Range {
        min: Box<Expr>,
        max: Box<Expr>,
        step: Option<Box<Expr>>,
    }, // range
}
