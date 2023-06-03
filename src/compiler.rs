use crate::expr::Expr;
use crate::opcode::OpCode;
use crate::scanner::{TokenType, Value};
use crate::stmt::Stmt;
use crate::vm::Type;

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

pub fn compile(stmts: Vec<Stmt>) -> Vec<OpCode> {
    let mut code: Vec<OpCode> = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Print(val) => {
                code.push(OpCode::Constant(val));
                code.push(OpCode::Print);
                code.push(OpCode::Pop);
            }
            Stmt::Block(stmts) => {
                code.push(OpCode::Scope);
                dump(&mut code, compile(stmts));
                code.push(OpCode::EndScope);
            }
            Stmt::Expression(expr) => dump(&mut code, compile_expr(expr)),
            Stmt::If {
                condition,
                block,
                elseblock,
            } => {}
            Stmt::Var { name, t, value } => {
                match value {
                    Some(value) => dump(&mut code, compile_expr(value)),
                    None => code.push(OpCode::Constant(Value::None)),
                }
                let t: Type = match t.tt {
                    TokenType::Int => Type::Int,
                    TokenType::Float => Type::Float,
                    TokenType::String => Type::String,
                    TokenType::Bool => Type::Bool,
                    _ => panic!("dont tell me right paren is a variable type now :skullemoji:"),
                };
                code.push(OpCode::Store(name.lexeme, t));
            }
            Stmt::While { condition, block } => {}
            Stmt::For {
                identifier,
                iterable,
                block,
            } => {}
            Stmt::Fn {
                return_t,
                name,
                params,
                body,
            } => {}
            Stmt::Return(expr) => {
                dump(&mut code, compile_expr(expr));
                code.push(OpCode::Return);
            }
        }
    }
    code
}

pub fn compile_expr(expr: Expr) -> Vec<OpCode> {
    let mut code: Vec<OpCode> = Vec::new();
    match expr {
        Expr::Assign { name, value } => {
            dump(&mut code, compile_expr(*value));
            code.push(OpCode::Store(name.lexeme, Type::Known));
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            dump(&mut code, compile_expr(*left));
            dump(&mut code, compile_expr(*right));
            code.push(bin(operator.tt));
        }
        Expr::Call { callee, arguments } => {
            code.push(OpCode::Call(match *callee {
                Expr::Variable(t) => t.lexeme,
                _ => unreachable!(),
            }));
            for arg_expr in arguments {
                dump(&mut code, compile_expr(arg_expr));
            }
            code.push(OpCode::Args);
        }
        Expr::Grouping(expression) => dump(&mut code, compile_expr(*expression)),
        Expr::Literal(x) => code.push(OpCode::Constant(x)),
        Expr::Range { min, max, step } => match step {
            Some(x) => {
                code.push(OpCode::Range(true));
                dump(&mut code, compile_expr(*min));
                dump(&mut code, compile_expr(*max));
                dump(&mut code, compile_expr(*x));
            }
            None => {
                code.push(OpCode::Range(false));
                dump(&mut code, compile_expr(*min));
                dump(&mut code, compile_expr(*max));
            }
        },
        Expr::Unary {
            operator,
            expression,
        } => {
            dump(&mut code, compile_expr(*expression));
            code.push(un(operator.tt));
        }
        Expr::Variable(name) => code.push(OpCode::Load(name.lexeme)),
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

pub fn dump(main: &mut Vec<OpCode>, se: Vec<OpCode>) {
    for i in se.into_iter() {
        main.push(i);
    }
}
