use crate::expr::Expr;
use crate::opcode::OpCode;
use crate::scanner::{TokenType, Value};
use crate::stmt::Stmt;
use crate::vm::Type;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new(stuff: (Vec<OpCode>, Vec<usize>)) -> Chunk {
        Chunk {
            code: stuff.0,
            lines: stuff.1,
        }
    }
    pub fn disassemble(&self) {
        for (num, instruction) in self.code.iter().enumerate() {
            println!("{:02} {:04} {}", self.lines[num], num, instruction);
        }
    }
    // pub fn disassemble_byte(&self, num: usize) {
    //     println!("{:04} {:?}", num, self.code[num]);
    // }
}

pub fn compile(stmts: Vec<Stmt>) -> (Vec<OpCode>, Vec<usize>) {
    let mut code: Vec<OpCode> = Vec::new();
    let mut lines: Vec<usize> = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Print(val, line) => {
                code.push(OpCode::Constant(val));
                lines.push(line);
                code.push(OpCode::Print);
                lines.push(line);
                code.push(OpCode::Pop);
                lines.push(line);
            }
            Stmt::Block(stmts, (start, end)) => {
                code.push(OpCode::Scope);
                lines.push(start);
                dump(&mut code, &mut lines, compile(stmts));
                code.push(OpCode::EndScope);
                lines.push(end);
            }
            Stmt::Expression(expr) => dump(&mut code, &mut lines, compile_expr(expr)),
            Stmt::If {
                condition,
                block,
                elseblock,
                lines: line,
            } => {
                dump(&mut code, &mut lines, compile_expr(condition));
                code.push(OpCode::LogicalNot); //jump if false
                lines.push(line.0);
                let b_vec: Vec<Stmt> = vec![*block];
                let blok = compile(b_vec);
                code.push(OpCode::JumpIf(blok.0.len() as i32));
                lines.push(line.0);
                dump(&mut code, &mut lines, blok);
                if elseblock.is_some() {
                    code.push(OpCode::LogicalNot); //jump if false
                    lines.push(line.1.unwrap());
                    let b_vec: Vec<Stmt> = vec![*elseblock.unwrap()];
                    let blok = compile(b_vec);
                    code.push(OpCode::JumpIf(blok.0.len() as i32));
                    lines.push(line.1.unwrap());
                    dump(&mut code, &mut lines, blok);
                }
            }
            Stmt::Var { name, t, value } => {
                match value {
                    Some(value) => dump(&mut code, &mut lines, compile_expr(value)),
                    None => {
                        code.push(OpCode::Constant(Value::None));
                        lines.push(name.line)
                    }
                }
                let t: Type = match t.tt {
                    TokenType::Int => Type::Int,
                    TokenType::Float => Type::Float,
                    TokenType::String => Type::String,
                    TokenType::Bool => Type::Bool,
                    _ => panic!("dont tell me right paren is a variable type now :skullemoji:"),
                };
                code.push(OpCode::Store(name.lexeme, t));
                lines.push(name.line)
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
            Stmt::Return(expr, line) => {
                dump(&mut code, &mut lines, compile_expr(expr));
                code.push(OpCode::Return);
                lines.push(line)
            }
        }
    }
    (code, lines)
}

pub fn compile_expr(expr: Expr) -> (Vec<OpCode>, Vec<usize>) {
    let mut code: Vec<OpCode> = Vec::new();
    let mut lines: Vec<usize> = Vec::new();

    match expr {
        Expr::Assign { name, value } => {
            dump(&mut code, &mut lines, compile_expr(*value));
            code.push(OpCode::Store(name.lexeme, Type::Known));
            lines.push(name.line)
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            dump(&mut code, &mut lines, compile_expr(*left));
            dump(&mut code, &mut lines, compile_expr(*right));
            code.push(bin(operator.tt));
            lines.push(operator.line)
        }
        Expr::Call { callee, arguments } => {
            let line;
            code.push(OpCode::Call(match *callee {
                Expr::Variable(t) => {
                    lines.push(t.line);
                    line = t.line;
                    t.lexeme
                }
                _ => unreachable!(),
            }));
            lines.push(line);
            for arg_expr in arguments {
                dump(&mut code, &mut lines, compile_expr(arg_expr));
            }
            code.push(OpCode::Args);
            lines.push(line);
        }
        Expr::Grouping(expression) => dump(&mut code, &mut lines, compile_expr(*expression)),
        Expr::Literal(x, line) => {
            code.push(OpCode::Constant(x));
            lines.push(line);
        }
        Expr::Range {
            min,
            max,
            step,
            line,
        } => match step {
            Some(x) => {
                code.push(OpCode::Range(true));
                lines.push(line);
                dump(&mut code, &mut lines, compile_expr(*min));
                dump(&mut code, &mut lines, compile_expr(*max));
                dump(&mut code, &mut lines, compile_expr(*x));
            }
            None => {
                code.push(OpCode::Range(false));
                lines.push(line);
                dump(&mut code, &mut lines, compile_expr(*min));
                dump(&mut code, &mut lines, compile_expr(*max));
            }
        },
        Expr::Unary {
            operator,
            expression,
        } => {
            dump(&mut code, &mut lines, compile_expr(*expression));
            code.push(un(operator.tt));
            lines.push(operator.line);
        }
        Expr::Variable(name) => {
            code.push(OpCode::Load(name.lexeme));
            lines.push(name.line)
        }
    }
    (code, lines)
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

pub fn dump(main: &mut Vec<OpCode>, lines: &mut Vec<usize>, se: (Vec<OpCode>, Vec<usize>)) {
    for i in se.0.into_iter() {
        main.push(i);
    }
    for i in se.1.into_iter() {
        lines.push(i);
    }
}
