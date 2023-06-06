use crate::{
    compiler::Chunk,
    opcode::OpCode,
    scanner::{TokenType, Value},
    KlangError,
};
use std::collections::HashMap;
pub type Function = (Vec<OpCode>, Vec<String>);
#[derive(Debug, Clone)]
pub struct VM<'a> {
    pub chunk: Chunk,
    pub global: Scope,
    pub index: i32,
    pub filename: &'a str,
    pub functions: HashMap<String, Function>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: Chunk, filename: &'a str) -> VM<'a> {
        VM {
            chunk,
            global: Scope::new(),
            index: 0,
            filename,
            functions: HashMap::new(),
        }
    }
    pub fn run(&mut self) {
        //executes the code on the chunk
        while self.index < self.chunk.code.len() as i32 {
            // println!("{:?}", self.chunk.code[self.index as usize]);
            // println!("{:?}", self.global);
            match self.chunk.code[self.index as usize].clone() {
                OpCode::Constant(x) => self.push(x),
                OpCode::Store(x) => self.set_var(x),
                OpCode::Load(x) => {
                    let var = match VM::get_var(&x, &mut self.global).0 {
                        Some(x) => x,
                        None => {
                            self.error(format!("variable \"{x}\" do not exist").as_str());
                            panic!()
                        }
                    };
                    self.push(var);
                }
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
                OpCode::LogicalAnd => self.bin_op(TokenType::And),
                OpCode::LogicalOr => self.bin_op(TokenType::Or),
                OpCode::LogicalNot => self.un_op(TokenType::Bang),
                OpCode::Negate => self.un_op(TokenType::Minus),
                OpCode::Jump(x) => {
                    if self.index + x > self.chunk.code.len() as i32 {
                        self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                    }
                    self.index += x;
                }
                OpCode::JumpIf(x, _) => {
                    if let Value::Bool(true) = self.top() {
                        if self.index + x > self.chunk.code.len() as i32 {
                            self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                        }
                        self.index += x;
                    }
                }
                OpCode::Call(x, y) => {
                    self.index += self.call(x, y, 0);
                }
                OpCode::NativeCall(x) => self.native_call(x),
                OpCode::Print => self.print(),
                OpCode::Args => self.error("args can only appear in the call function"),
                OpCode::Range(x) => self.range(x),
                OpCode::Scope => self.create_inner(),
                OpCode::EndScope => self.close_inner(),
                OpCode::Return => self.error("return can only appear inside functions"),
                OpCode::For => self.for_loop(),
                OpCode::Fn => self.function(),
                OpCode::Eof => {}
            }
            self.index += 1;
        }
    }
    fn execute_single(&mut self, opcode: &OpCode, mut index: i32, depth: i32) -> i32 {
        //executes the code on the chunk
        match opcode.clone() {
            OpCode::Constant(x) => self.push(x),
            OpCode::Store(x) => self.set_var(x),
            OpCode::Load(x) => {
                let var = match VM::get_var(&x, &mut self.global).0 {
                    Some(x) => x,
                    None => {
                        self.error(format!("variable \"{x}\" do not exist").as_str());
                        panic!()
                    }
                };
                self.push(var);
            }
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
            OpCode::LogicalAnd => self.bin_op(TokenType::And),
            OpCode::LogicalOr => self.bin_op(TokenType::Or),
            OpCode::LogicalNot => self.un_op(TokenType::Bang),
            OpCode::Negate => self.un_op(TokenType::Minus),
            OpCode::Jump(x) => {
                if index + x > self.chunk.code.len() as i32 {
                    self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                }
                index += x;
            }
            OpCode::JumpIf(x, y) => {
                if y {
                    if let Value::Bool(true) = self.pop() {
                        if index + x > self.chunk.code.len() as i32 {
                            self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                        }
                        index += x;
                    }
                } else if let Value::Bool(true) = self.top() {
                    if index + x > self.chunk.code.len() as i32 {
                        self.error("cannot jump out of bounds like ur dad jumped out of the 50th story window bozo");
                    }
                    index += x;
                }
            }
            OpCode::Call(x, y) => {
                index += self.call(x, y, depth + 1);
                println!("{:?}", self.global);
            }
            OpCode::NativeCall(x) => self.native_call(x),
            OpCode::Print => self.print(),
            OpCode::Args => self.error("args can only appear in the call function"),
            OpCode::Range(x) => self.range(x),
            OpCode::Scope => self.create_inner(),
            OpCode::EndScope => self.close_inner(),
            OpCode::Return => self.error("return can only appear inside functions"),
            OpCode::For => self.for_loop(),
            OpCode::Fn => self.function(),
            OpCode::Eof => {}
        };
        index
    }
    fn function(&mut self) {
        self.index += 1; //consume fn
        let mut args: Vec<String> = Vec::new();
        while match self.chunk.code[self.index as usize].clone() {
            OpCode::Store(x) => {
                args.push(x);
                true
            }
            _ => false,
        } {
            self.index += 1; //consume arg
        }
        let mut bytes: Vec<OpCode> = Vec::new();
        self.index += 1;
        let mut counter = 1;
        while counter != 0 {
            bytes.push(self.chunk.code[self.index as usize].clone());
            self.index += 1;
            if matches!(self.chunk.code[self.index as usize], OpCode::EndScope) {
                counter -= 1;
            }
            if matches!(self.chunk.code[self.index as usize], OpCode::Scope) {
                counter += 1;
            }
        }
        self.index += 1;
        match self.chunk.code[self.index as usize].clone() {
            OpCode::Store(x) => self.functions.insert(x, (bytes, args)),
            _ => {
                self.error("ksang made a little oopsy");
                panic!();
            }
        };
    }
    fn range(&mut self, cstep: bool) {
        if cstep {
            let step = match self.pop() {
                Value::Number(x) => x,
                _ => {
                    self.error("step is not a number");
                    panic!()
                }
            };
            let end = match self.pop() {
                Value::Number(x) => x,
                _ => {
                    self.error("end is not a number");
                    panic!()
                }
            };
            let start = match self.pop() {
                Value::Number(x) => x,
                _ => {
                    self.error("start is not a number");
                    panic!()
                }
            };
            let mut vec: Vec<Value> = Vec::new();
            for i in (start as usize..end as usize).step_by(step as usize) {
                vec.push(Value::Number(i as f64));
            }
            self.push(Value::Vec(vec))
        } else {
            let end = match self.pop() {
                Value::Number(x) => x,
                _ => {
                    self.error("end is not a number");
                    panic!()
                }
            };
            let start = match self.pop() {
                Value::Number(x) => x,
                _ => {
                    self.error("start is not a number");
                    panic!()
                }
            };
            let mut vec: Vec<Value> = Vec::new();
            for i in start as usize..end as usize {
                vec.push(Value::Number(i as f64));
            }
            self.push(Value::Vec(vec))
        }
    }
    fn for_loop(&mut self) {
        //opcodes list:
        //starts with for opcode
        //then calls self.range() to evaluate the iterable
        //Store(iden)
        //jumpif
        //block
        //jump
    }
    fn print(&mut self) {
        println!("{:?}", self.global);
        let mut print = match self.pop() {
            Value::String {
                string,
                printables: _,
            } => string,
            _ => {
                panic!()
            }
        };
        for _ in 0..self.count_braces(print.as_str()) {
            let repl = match self.pop() {
                Value::String {
                    string,
                    printables: _,
                } => string,
                Value::Number(x) => x.to_string(),
                Value::Bool(x) => x.to_string(),
                Value::Vec(x) => format!("{:?}", x),
                Value::None => "None".to_string(),
            };
            print = self.replace_last_braces(print.as_str(), repl.as_str());
        }
        println!("{}", print);
    }
    fn count_braces(&self, string: &str) -> usize {
        let mut count = 0;
        let mut braces = 0;
        for c in string.chars() {
            match c {
                '{' => braces += 1,
                '}' => {
                    if braces > 0 {
                        braces -= 1;
                        if braces == 0 {
                            count += 1;
                        }
                    }
                }
                _ => {}
            }
        }
        count
    }
    fn replace_last_braces(&self, string: &str, replacement: &str) -> String {
        if let Some((start, _)) = string.rmatch_indices("{}").next() {
            let mut modified = String::with_capacity(string.len() - 2 + replacement.len());
            modified.push_str(&string[..start]);
            modified.push_str(replacement);
            modified.push_str(&string[start + 2..]);
            modified
        } else {
            String::from(string)
        }
    }
    fn get_var(name: &str, scope: &mut Scope) -> (Option<Value>, bool) {
        //gets a variable from the most inner scope, if its not there searches on the outer scopes, return true when found the variable
        if scope.inner.is_some() {
            let i = VM::get_var(name, scope.inner.as_mut().unwrap());
            if !i.1 {
                return match scope.callframe.get(name) {
                    Some(val) => (Some(val.clone()), true),
                    None => (None, false),
                };
            }
            return i;
        }
        match scope.callframe.get(name) {
            Some(val) => (Some(val.clone()), true),
            None => (None, false),
        }
    }
    fn set_var(&mut self, name: String) {
        //sets a variable in the most inner scope, to the top value of the stack
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        scope.callframe.insert(name, scope.stack.pop().unwrap());
    }
    fn set_var_(&mut self, name: String, val: Value) {
        //sets a variable in the most inner scope, to the top value of the stack
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        scope.callframe.insert(name, val);
    }
    fn create_inner(&mut self) {
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        scope.inner = Some(Box::new(Scope::new()));
    }
    fn close_inner(&mut self) {
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.as_mut().unwrap().inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        scope.inner = None;
    }
    fn error(&self, msg: &str) {
        KlangError::error(
            KlangError::RuntimeError,
            msg,
            self.chunk.lines[self.index as usize],
            self.filename,
        );
    }

    fn bin_op(&mut self, operation: TokenType) {
        let pop2 = self.pop2();
        self.push(match operation {
            TokenType::Plus => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                _ => {
                    self.error("can only add numbers");
                    panic!()
                }
            },
            TokenType::Minus => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Number(y - x),
                _ => {
                    self.error("can only subtract numbers");
                    panic!()
                }
            },
            TokenType::Star => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Number(x * y),
                _ => {
                    self.error("can only multiply numbers");
                    panic!()
                }
            },
            TokenType::Slash => match pop2 {
                (Value::Number(x), Value::Number(y)) => {
                    if x == 0.0 {
                        self.error("division by zero");
                        panic!()
                    }
                    Value::Number(y / x)
                }
                _ => {
                    self.error("can only divide numbers");
                    panic!()
                }
            },
            TokenType::EqualEqual => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Bool(x == y),
                (Value::Bool(x), Value::Bool(y)) => Value::Bool(x == y),
                (Value::String { string: x, .. }, Value::String { string: y, .. }) => {
                    Value::Bool(x == y)
                }
                _ => Value::Bool(false),
            },
            TokenType::BangEqual => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Bool(x != y),
                (Value::Bool(x), Value::Bool(y)) => Value::Bool(x != y),
                (Value::String { string: x, .. }, Value::String { string: y, .. }) => {
                    Value::Bool(x != y)
                }
                _ => Value::Bool(true),
            },
            TokenType::Less => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Bool(x > y),
                _ => {
                    self.error("can only compare numbers");
                    panic!()
                }
            },
            TokenType::LessEqual => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Bool(x >= y),
                _ => {
                    self.error("can only compare numbers");
                    panic!()
                }
            },
            TokenType::Greater => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Bool(x < y),
                _ => {
                    self.error("can only compare numbers");
                    panic!()
                }
            },
            TokenType::GreaterEqual => match pop2 {
                (Value::Number(x), Value::Number(y)) => Value::Bool(x <= y),
                _ => {
                    self.error("can only compare numbers");
                    panic!()
                }
            },
            TokenType::And => match pop2 {
                (Value::Bool(x), Value::Bool(y)) => Value::Bool(x && y),
                _ => {
                    self.error("can only perform logical AND on bool values");
                    panic!()
                }
            },
            TokenType::Or => match pop2 {
                (Value::Bool(x), Value::Bool(y)) => Value::Bool(x || y),
                _ => {
                    self.error("can only perform logical OR on bool values");
                    panic!()
                }
            },
            _ => {
                self.error("unsupported binary operation");
                panic!()
            }
        });
    }
    fn un_op(&mut self, operation: TokenType) {
        let pop = self.pop();
        self.push(match operation {
            TokenType::Bang => match pop {
                Value::Bool(x) => Value::Bool(!x),
                _ => {
                    self.error("can only use ! on bools");
                    panic!()
                }
            },
            TokenType::Minus => match pop {
                Value::Number(x) => Value::Number(-x),
                _ => {
                    self.error("can only use minus on ints and floats");
                    panic!()
                }
            },
            _ => {
                self.error("unsupported unary operation");
                panic!()
            }
        })
    }
    fn call(&mut self, callee: String, mut index: i32, depth: i32) -> i32 {
        index += 1;
        let mut counter = 1;
        while !matches!(self.chunk.code[index as usize], OpCode::Args) {
            let opcode = self.chunk.code[index as usize].clone();
            index = self.execute_single(&opcode, index, depth) + 1;
            counter += 1;
            println!("{:?} {:?}", self.chunk.code[index as usize], self.global);
        }
        let fun = match self.functions.remove(&callee) {
            Some(x) => x,
            None => {
                self.error("please call a real function next time stupid ass mf");
                panic!()
            }
        };
        self.create_inner();
        self.functions.insert(callee, fun.clone());
        for i in fun.1.into_iter().rev() {
            let mut scope = &mut self.global;
            for _ in 0..depth {
                scope = scope.inner.as_mut().unwrap();
            }
            let pop = match scope.stack.pop() {
                Some(x) => x,
                None => {
                    self.error("provided too many arguments");
                    panic!();
                }
            };
            self.set_var_(i, pop);
        }
        let mut i: i32 = 0;
        while fun.0.len() > i as usize {
            if matches!(fun.0[i as usize], OpCode::Return) {
                let val = self.pop();
                self.close_inner();
                let mut scope = &mut self.global;
                for _ in 0..depth {
                    scope = scope.inner.as_mut().unwrap();
                }
                scope.stack.push(val);
                return counter;
            }
            i += self.execute_single(&fun.0[i as usize], index, depth) - index + 1;
            println!("{} {:?} {:?}", depth, fun.0[i as usize], self.global);
        }
        self.close_inner();
        counter
    }
    fn native_call(&mut self, callee: String) {}

    fn pop2(&mut self) -> (Value, Value) {
        (self.pop(), self.pop())
    }
    fn pop(&mut self) -> Value {
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        if scope.stack.is_empty() {
            self.error("stack overflow (cant pop an empty stack)");
            panic!()
        }
        scope.stack.pop().unwrap()
    }
    fn top(&mut self) -> Value {
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        if scope.stack.is_empty() {
            self.error("stack overflow (cant top an empty stack)");
            panic!()
        }
        let val = scope.stack.pop().unwrap();
        scope.stack.push(val.clone());
        val
    }
    fn push(&mut self, v: Value) {
        let mut scope: &mut Scope = &mut self.global;
        while scope.inner.is_some() {
            scope = scope.inner.as_mut().unwrap();
        }
        scope.stack.push(v);
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub callframe: HashMap<String, Value>,
    pub inner: Option<Box<Scope>>,
    pub stack: Vec<Value>,
}
impl Scope {
    pub fn new() -> Self {
        Self {
            callframe: HashMap::new(),
            inner: None,
            stack: Vec::new(),
        }
    }
}
