use super::{
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
        name: Token,
        params: Vec<Token>,
        body: Box<Stmt>,
    },
    Return(Option<Expr>, usize),
}
