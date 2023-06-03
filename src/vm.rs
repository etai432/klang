use crate::{
    compiler::{compile, Chunk},
    scanner::Value,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VM {
    pub stack: Vec<Value>,
    pub chunk: Chunk,
    pub callframe: HashMap<&'static str, (Value, Type)>,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
            callframe: HashMap::new(),
        }
    }
    pub fn interpret(&mut self, source: String) {
        self.run();
    }
    pub fn run(&mut self) {}
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Known,
}
