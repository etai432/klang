use crate::expr::Expr;
use crate::opcode::OpCode;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new(code: Vec<OpCode>) -> Chunk {
        Chunk { code }
    }
    pub fn disassemble(&self) {
        for (num, instruction) in self.code.iter().enumerate() {
            println!("{:04} {}", num, instruction);
        }
    }
    // pub fn disassemble_byte(&self, num: usize) {
    //     println!("{:04} {:?}", num, self.code[num]);
    // }
}

pub fn compile(expr: Expr) -> Vec<OpCode> {
    compile_expr(expr)
}

pub fn compile_expr(expr: Expr) -> Vec<OpCode> {
    let mut code: Vec<OpCode> = Vec::new();
    match expr {
        Expr::Assign { name, value } => {}
        Expr::Binary {
            left,
            operator,
            right,
        } => {}
        Expr::Call { callee, arguments } => {}
        Expr::Grouping { expression } => {
            for i in compile_expr(*expression) {
                code.push(i);
            }
        }
        Expr::Literal(x) => code.push(OpCode::Constant(x)),
        Expr::Range { min, max, step } => {}
        Expr::Unary {
            operator,
            expression,
        } => {}
        Expr::Variable { name } => {}
    }
    code
}

pub fn dump(mut main: Vec<OpCode>, se: Vec<OpCode>) -> Vec<OpCode> {
    for i in se.into_iter() {
        main.push(i);
    }
    main
}
