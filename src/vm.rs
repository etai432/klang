use crate::{
    compiler::{compile, Chunk},
    opcode::OpCode,
    scanner::{TokenType, Value},
    KlangError,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VM<'a> {
    pub stack: Vec<Value>,
    pub chunk: Chunk,
    pub global: Scope,
    pub index: i32,
    pub filename: &'a str,
}

impl<'a> VM<'a> {
    pub fn new(chunk: Chunk, filename: &'a str) -> VM<'a> {
        VM {
            chunk,
            stack: Vec::new(),
            global: Scope::new(),
            index: 0,
            filename,
        }
    }
    pub fn run(&mut self) {
        //executes the code on the chunk
        while self.index < self.chunk.code.len() {
            match self.chunk.code[self.index as usize] {
                OpCode::Constant(x) => self.stack.push(x),
                OpCode::Store(x, y) => self.set_var(x, y),
                OpCode::Load(x) => self.get_var(x),
                OpCode::Add => self.bin_op(TokenType::Plus),
                OpCode::Subtract => self.bin_op(TokenType::Minus),
                OpCode::Multiply => self.bin_op(TokenType::Star),
                OpCode::Divide => self.bin_op(TokenType::Slash),
                OpCode::EqualEqual => self.bin_op(TokenType::EqualEqual),
                OpCode::NotEqual => self.bin_op(TokenType::BangEqual),
                OpCode::Less => self.bin_op(TokenType::Less),
                OpCode::LessEqual => self.bin_op(TokenType::LessEqual),
                OpCode::Greater => self.bin_op(TokenType::Greater),
                OpCode::GreaterEqual => self.bin_op(TokenType::GreaterEqual),
                OpCode::LogicalAnd => self.un_op(TokenType::And),
                OpCode::LogicalOr => self.un_op(TokenType::Or),
                OpCode::LogicalNot => self.un_op(TokenType::Bang),
                OpCode::Negate => self.un_op(TokenType::Minus),
                OpCode::Jump(x) => {
                    if self.index + x > self.chunk.code.len() as i32 {
                        self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                    }
                    self.index += x;
                }
                OpCode::JumpIf(x) => {
                    if let Value::Bool(true) = self.pop() {
                        if self.index + x > self.chunk.code.len() as i32 {
                            self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                        }
                        self.index += x;
                    }
                }
                OpCode::Call(x) => self.call(x),
                OpCode::NativeCall(x) => self.native_call(x),
            }
        }
    }
    fn get_var(&self, name: String) {
        //gets a variable from the most inner scope, if its not there searches on the outer scopes
    }
    fn set_var(&mut self, name: String, r#type: Type) {
        //sets a variable in the most inner scope
    }
    fn error(&self, msg: &str) {
        KlangError::error(
            KlangError::RuntimeError,
            msg,
            self.chunk.lines[self.index as usize],
            self.filename,
        );
    }

    fn bin_op(&mut self, operation: TokenType) {}
    fn un_op(&mut self, operation: TokenType) {}
    fn call(&mut self, callee: String) {}
    fn native_call(&mut self, callee: String) {}

    fn pop2(&mut self) -> (Value, Value) {
        (self.pop(), self.pop())
    }
    fn pop(&mut self) -> Value {
        if self.stack.is_empty() {
            self.error("stack overflow (cant pop an empty stack)");
            panic!()
        }
        self.stack.pop().unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    None,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub callframe: HashMap<&'static str, (Value, Type)>,
    pub inner: Option<Box<Scope>>,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            callframe: HashMap::new(),
            inner: None,
        }
    }
}
