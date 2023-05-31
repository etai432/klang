use crate::{expr::*, scanner::Value};
pub enum Stmt {
    Print { value: Value },
    Block,
    Expression,
    If,
    Var,
    While,
    For,
    Fn,
    Return,
}
