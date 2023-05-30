use crate::scanner::{Token, Value};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    }, //assignment
    Binary(Binary), //binary operations (+, -, .., ==, !=, <=, ..)
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Grouping(Grouping), // "(" expression ")"
    Literal(Value),
    Logical(Logical), // and / or
    Unary(Unary),     // ! or - (negate)
    Variable {
        name: Token,
    },
    Range(Range), // range
}
