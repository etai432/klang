use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Return,
    Constant(f64),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Return => write!(f, "OP_RETURN"),
            OpCode::Constant(x) => write!(f, "OP_CONSTANT {}", x),
            OpCode::Negate => write!(f, "OP_NEGATE"),
            OpCode::Add => write!(f, "OP_ADD"),
            OpCode::Subtract => write!(f, "OP_SUBTRACT"),
            OpCode::Multiply => write!(f, "OP_MULTIPLY"),
            OpCode::Divide => write!(f, "OP_DIVIDE"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk { code: Vec::new() }
    }
    pub fn write_chunk(&mut self, byte: OpCode) {
        self.code.push(byte);
    }
    pub fn disassemble(&self) {
        for (num, instruction) in self.code.iter().enumerate() {
            println!("{:04} {}", num, instruction);
        }
    }
    pub fn disassemble_byte(&self, num: usize) {
        println!("{:04} {}", num, self.code[num]);
    }
}

pub fn compile() {}
