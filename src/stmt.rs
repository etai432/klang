use crate::{
    expr::*,
    scanner::{Token, Value},
};
#[derive(Clone, Debug)]
pub enum Stmt {
    Print(Value, usize),
    Block(Vec<Stmt>, (usize, usize)),
    Expression(Expr),
    If {
        condition: Expr,
        block: Box<Stmt>,
        elseblock: Option<Box<Stmt>>,
        lines: (usize, Option<usize>),
    },
    Var {
        name: Token,
        t: Token,
        value: Option<Expr>,
    },
    While {
        condition: Expr,
        block: Box<Stmt>,
        line: usize,
    },
    For {
        identifier: Token,
        iterable: Expr,
        block: Box<Stmt>,
        line: usize,
    },
    Fn {
        return_t: Option<Token>,
        name: Token,
        params: Vec<(Token, Token)>,
        body: Box<Stmt>,
    },
    Return(Expr, usize),
}
