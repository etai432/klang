use crate::scanner::{Token, Value};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: Token,
        t: Type,
        value: Box<Expr>,
    }, //assignment
    Binary(Binary), //binary operations (+, -, .., ==, !=, <=, ..)
    Call(Call),
    Grouping(Grouping), // "(" expression ")"
    Literal(Value),
    Logical(Logical), // and / or
    Unary(Unary),     // ! or - (negate)
    Variable(Variable),
    Range(Range), // range
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}
