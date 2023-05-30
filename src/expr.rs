use crate::scanner::{Token, Value};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign { name: Token, value: Box<Expr> }, //assignment
    Binary(Binary),                           //binary operations (+, -, .., ==, !=, <=, ..)
    Call(Call),
    Grouping(Grouping), // "(" expression ")"
    Literal(Value),
    Logical(Logical), // and / or
    Unary(Unary),     // ! or - (negate)
    Variable(Variable),
    Range(Range), // range
}
