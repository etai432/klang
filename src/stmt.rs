use crate::{
    expr::*,
    scanner::{Token, TokenType, Value},
};
pub enum Stmt {
    Print {
        value: Value,
    },
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    If {
        condition: Expr,
        block: Box<Stmt>,
        elseblock: Option<Box<Stmt>>,
    },
    Var {
        name: TokenType,
        value: Option<Expr>,
    },
    While {
        condition: Expr,
        block: Box<Stmt>,
    },
    For,
    Fn,
    Return,
}
