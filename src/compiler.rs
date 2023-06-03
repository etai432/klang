use crate::error::KlangError;
use crate::expr::Expr;
use crate::opcode::OpCode;
use crate::scanner::TokenType;
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
        } => {
            code = dump(code, compile_expr(*left));
            code = dump(code, compile_expr(*right));
            code.push(bin(operator.tt));
        }

        Expr::Call { callee, arguments } => {}
        Expr::Grouping { expression } => code = dump(code, compile_expr(*expression)),
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

pub fn bin(operator: TokenType) -> OpCode {
    match operator {
        TokenType::Plus => OpCode::Add,
        TokenType::Minus => OpCode::Subtract,
        TokenType::Star => OpCode::Multiply,
        TokenType::Slash => OpCode::Divide,
        TokenType::EqualEqual => OpCode::EqualEqual,
        TokenType::BangEqual => OpCode::NotEqual,
        TokenType::Less => OpCode::Less,
        TokenType::LessEqual => OpCode::LessEqual,
        TokenType::Greater => OpCode::Greater,
        TokenType::GreaterEqual => OpCode::GreaterEqual,
        TokenType::And => OpCode::LogicalAnd,
        TokenType::Or => OpCode::LogicalOr,
        _ => panic!("how did you even get here?"),
    }
}

pub fn un(operator: TokenType) -> OpCode {
    match operator {
        TokenType::Minus => OpCode::Negate,
        TokenType::Bang => OpCode::LogicalNot,
        _ => panic!("how did you even get here?"),
    }
}

pub fn dump(mut main: Vec<OpCode>, se: Vec<OpCode>) -> Vec<OpCode> {
    for i in se.into_iter() {
        main.push(i);
    }
    main
}
