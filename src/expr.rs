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
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    }, // "(" expression ")"
    Literal(Value),

    Unary {
        operator: Token,
        expression: Box<Expr>,
    }, // ! or - (negate)
    Variable {
        name: Token,
    },
    Range {
        min: i64,
        max: i64,
        step: Option<i64>,
    }, // range
}
