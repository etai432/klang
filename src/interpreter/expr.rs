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
        native: bool,
    },
    Grouping(Box<Expr>), // "(" expression ")"
    Literal(Value, usize),

    Unary {
        operator: Token,
        expression: Box<Expr>,
    }, // ! or - (negate)
    Variable(Token),
    Range {
        min: Box<Expr>,
        max: Box<Expr>,
        step: Option<Box<Expr>>,
        line: usize,
    }, // range
    Vec(Vec<Expr>),
}
