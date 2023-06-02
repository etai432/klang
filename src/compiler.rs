use crate::opcode::OpCode;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

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
            println!("{:04} {:?}", num, instruction);
        }
    }
    pub fn disassemble_byte(&self, num: usize) {
        println!("{:04} {:?}", num, self.code[num]);
    }
}

pub fn compile() {}

fn save_bytecode_to_file(bytecode: &[u8], filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(bytecode)?;
    Ok(())
}
