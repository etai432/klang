use crate::scanner::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(Value),         //Load a constant value onto the stack
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
    NativeCall(String, i32), // Calls a native function or external function.
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
            OpCode::NativeCall(x, y) => write!(f, "NativeCall {} {}", x, y),
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
