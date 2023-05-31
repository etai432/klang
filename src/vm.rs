use crate::compiler::{compile, Chunk, OpCode};

#[derive(Debug, Clone)]
pub struct VM {
    pub stack: Vec<f64>,
    pub chunk: Chunk,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }
    pub fn interpret(&mut self, source: String) {
        compile();
        self.run();
    }
    pub fn run(&mut self) {}
}
