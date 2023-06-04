use crate::{
    compiler::{compile, Chunk},
    scanner::Value,
    KlangError,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VM<'a> {
    pub stack: Vec<Value>,
    pub chunk: Chunk,
    pub global: Scope,
    pub index: usize,
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
    }
    fn get_var(&self) {
        //gets a variable from the most inner scope, if its not there searches on the outer scopes
    }
    fn set_var(&mut self) {
        //sets a variable in the most inner scope
    }
    fn error(&self, msg: &str) {
        KlangError::error(
            KlangError::RuntimeError,
            msg,
            self.chunk.lines[self.index] as usize,
            self.filename,
        );
    }
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
