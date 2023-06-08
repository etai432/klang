use super::opcode::OpCode;
use crate::interpreter::expr::Expr;
use crate::interpreter::stmt::Stmt;
use crate::scanner::{TokenType, Value};

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
}

pub fn compile(stmts: Vec<Stmt>) -> (Vec<OpCode>, Vec<usize>) {
    let mut code: Vec<OpCode> = Vec::new();
    let mut lines: Vec<usize> = Vec::new();
    for stmt in stmts {
        match stmt {
            Stmt::Print(x, line) => {
                match x {
                    Value::String { string, printables } => {
                        for i in printables {
                            dump(&mut code, &mut lines, compile_expr(i))
                        }
                        code.push(OpCode::Constant(Value::String {
                            string,
                            printables: Vec::new(),
                        }));
                        lines.push(line);
                    }
                    _ => {
                        code.push(OpCode::Constant(x));
                        lines.push(line);
                    }
                };
                code.push(OpCode::Print);
                lines.push(line);
            }
            Stmt::Block(stmts, (start, end)) => {
                code.push(OpCode::Scope);
                lines.push(start);
                dump(&mut code, &mut lines, compile(stmts));
                code.pop();
                lines.pop();
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
                code.push(OpCode::JumpIf(blok.0.len() as i32 - 1, elseblock.is_none()));
                lines.push(line.0);
                dump(&mut code, &mut lines, blok);
                code.pop();
                lines.pop();
                if elseblock.is_some() {
                    code.push(OpCode::LogicalNot); //jump if true
                    lines.push(line.1.unwrap());
                    let b_vec: Vec<Stmt> = vec![*elseblock.unwrap()];
                    let blok = compile(b_vec);
                    code.push(OpCode::JumpIf(blok.0.len() as i32 - 1, true));
                    lines.push(line.1.unwrap());
                    dump(&mut code, &mut lines, blok);
                    code.pop();
                    lines.pop();
                }
            }
            Stmt::Var { name, value } => {
                match value {
                    Some(value) => dump(&mut code, &mut lines, compile_expr(value)),
                    None => {
                        code.push(OpCode::Constant(Value::None));
                        lines.push(name.line)
                    }
                }
                code.push(OpCode::Store(name.lexeme));
                lines.push(name.line)
            }
            Stmt::While {
                condition,
                block,
                line,
            } => {
                let condition = compile_expr(condition);
                let con_len = condition.0.len() as i32;
                dump(&mut code, &mut lines, condition);
                code.push(OpCode::LogicalNot); //jump if false
                lines.push(line);
                let b_vec: Vec<Stmt> = vec![*block];
                let blok = compile(b_vec);
                let block_len = blok.0.len() as i32;
                code.push(OpCode::JumpIf(block_len, true));
                lines.push(line);
                dump(&mut code, &mut lines, blok);
                code.pop();
                lines.pop();
                code.push(OpCode::Jump(-(block_len + con_len + 2)));
                lines.push(line);
            }
            Stmt::For {
                identifier,
                iterable,
                block,
                line,
            } => {
                dump(&mut code, &mut lines, compile_expr(iterable));
                code.push(OpCode::For);
                lines.push(line);
                code.push(OpCode::Scope);
                lines.push(line);
                code.push(OpCode::Store(identifier.lexeme));
                lines.push(line);
                let b_vec: Vec<Stmt> = vec![*block];
                let mut blok = compile(b_vec);
                blok.0.remove(0);
                blok.1.remove(0);
                let block_len = blok.0.len() as i32;
                code.push(OpCode::JumpIf(block_len, true));
                lines.push(line);
                dump(&mut code, &mut lines, blok);
                code.pop();
                lines.pop();
                code.push(OpCode::Jump(-(block_len + 4)));
                lines.push(line);
            }
            Stmt::Fn { name, params, body } => {
                code.push(OpCode::Fn);
                lines.push(name.line);
                for i in params {
                    code.push(OpCode::Store(i.lexeme));
                    lines.push(name.line);
                }
                let b_vec: Vec<Stmt> = vec![*body];
                dump(&mut code, &mut lines, compile(b_vec));
                code.pop();
                lines.pop();
                code.push(OpCode::Store(name.lexeme));
                lines.push(name.line);
            }
            Stmt::Return(expr, line) => match expr {
                Some(expr) => {
                    dump(&mut code, &mut lines, compile_expr(expr));
                    code.push(OpCode::Return(true));
                    lines.push(line)
                }
                None => {
                    code.push(OpCode::Return(false));
                    lines.push(line)
                }
            },
        }
    }
    code.push(OpCode::Eof);
    lines.push(0);
    for (i, op) in code.clone().into_iter().enumerate() {
        if matches!(op, OpCode::Call(_)) {
            if let OpCode::Call(x) = &code[i] {
                code[i] = OpCode::Call(x.clone());
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
            code.push(OpCode::Store(name.lexeme));
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
        Expr::Call {
            callee,
            arguments,
            native,
        } => {
            let line;
            let len = arguments.len() as i32;
            for arg_expr in arguments {
                dump(&mut code, &mut lines, compile_expr(arg_expr));
            }
            if native {
                code.push(OpCode::NativeCall(
                    match *callee {
                        Expr::Variable(t) => {
                            lines.push(t.line);
                            line = t.line;
                            t.lexeme
                        }
                        _ => unreachable!(),
                    },
                    len,
                ));
            } else {
                code.push(OpCode::Call(match *callee {
                    Expr::Variable(t) => {
                        lines.push(t.line);
                        line = t.line;
                        t.lexeme
                    }
                    _ => unreachable!(),
                }));
            }
            lines.push(line);
        }
        Expr::Grouping(expression) => dump(&mut code, &mut lines, compile_expr(*expression)),
        Expr::Literal(x, line) => match x {
            Value::String { string, printables } => {
                for i in printables {
                    dump(&mut code, &mut lines, compile_expr(i))
                }
                code.push(OpCode::Constant(Value::String {
                    string,
                    printables: Vec::new(),
                }));
                lines.push(line);
            }
            _ => {
                code.push(OpCode::Constant(x));
                lines.push(line);
            }
        },
        Expr::Range {
            min,
            max,
            step,
            line,
        } => match step {
            Some(x) => {
                dump(&mut code, &mut lines, compile_expr(*min));
                dump(&mut code, &mut lines, compile_expr(*max));
                dump(&mut code, &mut lines, compile_expr(*x));
                code.push(OpCode::Range(true));
                lines.push(line);
            }
            None => {
                dump(&mut code, &mut lines, compile_expr(*min));
                dump(&mut code, &mut lines, compile_expr(*max));
                code.push(OpCode::Range(false));
                lines.push(line);
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
        Expr::Vec(vec) => {
            let len = vec.len();
            for i in vec {
                dump(&mut code, &mut lines, compile_expr(i));
            }
            code.push(OpCode::Iterable(len as i32));
            lines.push(0);
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
        TokenType::Modulo => OpCode::Modulo,
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
