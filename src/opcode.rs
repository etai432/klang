use crate::scanner::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(Value),    //Load a constant value onto the stack
    Store(String), // Store the value from the top of the stack into the variable in the hashtable.
    Load(String),  //Load the value of the variable from the hashtable onto the stack
    Add,           // Performs addition on the last two values on the stack.
    Subtract,      // Performs subtraction on the last two values on the stack.
    Multiply,      // Performs multiplication on the last two values on the stack.
    Divide,        // Performs division on the last two values on the stack.
    Modulo,        // sex
    EqualEqual,    // Compares equality between the last two values on the stack.
    NotEqual,      // Compares inequality between the last two values on the stack.
    Less,          // Checks if the second-to-last value on the stack is less than the last value.
    LessEqual, // Checks if the second-to-last value on the stack is less than or equal to the last value.
    Greater,   // Checks if the second-to-last value on the stack is greater than the last value.
    GreaterEqual, // Checks if the second-to-last value on the stack is greater than or equal to the last value.
    LogicalAnd,   // Performs logical AND operation on the last two boolean values on the stack.
    LogicalOr,    // Performs logical OR operation on the last two boolean values on the stack.
    LogicalNot,   // Negates the last boolean value on the stack.
    Negate,       // Negates the last numeric value on the stack.
    Jump(i32),    // Unconditionally jumps to a specified instruction address.
    JumpIf(i32, bool), // Jumps to a specified instruction address if the last value on the stack is true.
    Call(String),      // Calls a function at a specified instruction address.
    NativeCall(String), // Calls a native function or external function.
    Print,             // Prints the last value on the stack to the console or output stream.
    Range(bool),
    Scope,
    EndScope,
    EndFn,
    Return(bool),
    For,
    Fn,
    Iterable(i32),
    Eof,
}

// impl OpCode {
//     fn to_u8(&self) -> u8 {
//         match self {
//             OpCode::Constant(constant) => 0x00,
//             OpCode::Store(name, r#type) => 0x01,
//             OpCode::Load(name) => 0x02,
//             OpCode::Add => 0x03,
//             OpCode::Subtract => 0x04,
//             OpCode::Multiply => 0x05,
//             OpCode::Divide => 0x06,
//             OpCode::EqualEqual => 0x07,
//             OpCode::NotEqual => 0x08,
//             OpCode::Less => 0x09,
//             OpCode::LessEqual => 0x10,
//             OpCode::Greater => 0x11,
//             OpCode::GreaterEqual => 0x12,
//             OpCode::LogicalAnd => 0x13,
//             OpCode::LogicalOr => 0x14,
//             OpCode::LogicalNot => 0x15,
//             OpCode::Negate => 0x16,
//             OpCode::Jump(x) => 0x17,
//             OpCode::JumpIf(x) => 0x18,
//             OpCode::Call(x) => 0x19,
//             OpCode::NativeCall(x) => 0x20,
//             OpCode::Print => 0x21,
//             OpCode::Args => 0x22,
//             OpCode::Range(x) => 0x23,
//             OpCode::Pop => 0x24,
//             OpCode::Scope => 0x25,
//             OpCode::EndScope => 0x26,
//             OpCode::Return => 0x27,
//             OpCode::For => 0x28,
//             OpCode::Fn(x) => 0x29,
//             OpCode::Eof => 0x30,
//         }
//     }

//     fn fromu8(byte: u8) -> OpCode {
//         match byte {
//             0x00 => OpCode::Constant(Value::Default),
//             0x01 => OpCode::Store,
//             0x02 => OpCode::Load,
//             0x03 => OpCode::Add,
//             0x04 => OpCode::Subtract,
//             0x05 => OpCode::Multiply,
//             0x06 => OpCode::Divide,
//             0x07 => OpCode::EqualEqual,
//             0x08 => OpCode::NotEqual,
//             0x09 => OpCode::Less,
//             0x10 => OpCode::LessEqual,
//             0x11 => OpCode::Greater,
//             0x12 => OpCode::GreaterEqual,
//             0x13 => OpCode::LogicalAnd,
//             0x14 => OpCode::LogicalOr,
//             0x15 => OpCode::LogicalNot,
//             0x16 => OpCode::Negate,
//             0x17 => OpCode::Jump,
//             0x18 => OpCode::JumpIf,
//             0x19 => OpCode::Call,
//             0x20 => OpCode::NativeCall,
//             0x21 => OpCode::Print,
//             0x22 => OpCode::Args,
//             0x23 => OpCode::Range,
//             0x24 => OpCode::Pop,
//             0x25 => OpCode::Scope,
//             0x26 => OpCode::EndScope,
//             0x27 => OpCode::Return,
//             0x28 => OpCode::For,
//             0x29 => OpCode::Fn,
//             0x30 => OpCode::Eof,
//             _ => panic!("sex"),
//         }
//     }
// }

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(constant) => write!(f, "Constant {}", constant),
            OpCode::Store(name) => write!(f, "Store {}", name),
            OpCode::Load(name) => write!(f, "Load {}", name),
            OpCode::Add => write!(f, "Add"),
            OpCode::Subtract => write!(f, "Subtract"),
            OpCode::Multiply => write!(f, "Multiply"),
            OpCode::Divide => write!(f, "Divide"),
            OpCode::Modulo => write!(f, "Modulo"),
            OpCode::EqualEqual => write!(f, "EqualEqual"),
            OpCode::NotEqual => write!(f, "NotEqual"),
            OpCode::Less => write!(f, "Less"),
            OpCode::LessEqual => write!(f, "LessEqual"),
            OpCode::Greater => write!(f, "Greater"),
            OpCode::GreaterEqual => write!(f, "GreaterEqual"),
            OpCode::LogicalAnd => write!(f, "LogicalAnd"),
            OpCode::LogicalOr => write!(f, "LogicalOr"),
            OpCode::LogicalNot => write!(f, "LogicalNot"),
            OpCode::Negate => write!(f, "Negate"),
            OpCode::Jump(x) => write!(f, "Jump {}", x),
            OpCode::JumpIf(x, y) => write!(f, "JumpIf {} {}", x, y),
            OpCode::Call(x) => write!(f, "Call {}", x),
            OpCode::NativeCall(x) => write!(f, "NativeCall {}", x),
            OpCode::Print => write!(f, "Print"),
            OpCode::Range(x) => write!(f, "Range {}", x),
            OpCode::Scope => write!(f, "Scope"),
            OpCode::EndScope => write!(f, "EndScope"),
            OpCode::EndFn => write!(f, "EndFn"),
            OpCode::Return(x) => write!(f, "Return {}", x),
            OpCode::For => write!(f, "For"),
            OpCode::Fn => write!(f, "Fn"),
            OpCode::Iterable(x) => write!(f, "Iterable {}", x),
            OpCode::Eof => write!(f, "Eof"),
        }
    }
}
