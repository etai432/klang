use error::KlangError;
use opcode::OpCode;
use scanner::Token;
use std::path::{Path, PathBuf};
use std::{env, fs, fs::File};
mod coder;
mod compiler;
mod error;
mod expr;
mod opcode;
mod parser;
mod scanner;
mod stmt;
mod vm;

macro_rules! timeit {
    ($($todo: stmt), *) => {
        use std::time::SystemTime;
        let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        $(
            $todo
        )*
        let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let sec = end.as_secs() - start.as_secs();
        if sec >= 1 {
            if sec > 60 {
                println!("timeit result: {} minutes and {} seconds", sec % 60, sec / 60);
            }
            else {
                println!("timeit results: {} seconds", sec);
            }
        }
        else {
            let millis = end.as_millis() - start.as_millis();
            if millis >= 1 {
                println!("timeit results: {} milliseconds", millis);
            }
            else {
                let micros = end.as_micros() - start.as_micros();
                if micros >= 1 {
                    println!("timeit results: {} microseconds", micros);
                }
                else{
                    println!("timeit results: {} nanoseconds", end.as_nanos() - start.as_nanos())
                }
            }
        }
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: expected file path | use -c at the end to compile");
        std::process::exit(1);
    }
    let filename = &args[1];
    let path = Path::new(&filename);
    let relfilename = path.file_name().unwrap().to_str().unwrap();
    //run
    if args.len() == 2 {
        if !filename.ends_with(".klang") {
            KlangError::error(
                KlangError::RuntimeError,
                "file must have a \".klang\" extension!",
                0,
                relfilename,
            );
            panic!("sex");
        }
        if fs::metadata(filename).is_err() {
            KlangError::error(
                KlangError::RuntimeError,
                format!("File {filename} is not a file!").as_str(),
                0,
                relfilename,
            );
            std::process::exit(1);
        } else {
            run_file(filename, relfilename);
        }
    }
    //compile | run
    if args.len() == 3 {
        if fs::metadata(filename).is_err() {
            KlangError::error(
                KlangError::RuntimeError,
                format!("File {filename} is not a file!").as_str(),
                0,
                relfilename,
            );
            std::process::exit(1);
        }
        if args[2] == "-c" {
            if filename.ends_with(".klang") {
                compile_file(filename, relfilename);
            } else {
                KlangError::error(
                    KlangError::RuntimeError,
                    "file must have a \".klang\" extension!",
                    0,
                    relfilename,
                );
                panic!("sex");
            }
        } else if args[2] == "-r" {
            if filename.ends_with(".klc") {
                run_compiled(filename, relfilename);
            } else {
                KlangError::error(
                    KlangError::RuntimeError,
                    "file must have a \".klc\" extension!",
                    0,
                    relfilename,
                );
                panic!("sex");
            }
        } else {
            KlangError::error(
                KlangError::RuntimeError,
                "use cargo run path -c | -r to compile | run",
                0,
                relfilename,
            );
        }
    }
}

fn run_file(path: &str, relfilename: &str) {
    let source = fs::read_to_string(path).expect("failed to read file");
    let mut scanner = scanner::Scanner::new(&source, relfilename);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens, relfilename);
    let ast = parser.parse();
    println!("{:?}\n", ast);
    let chunk = compiler::Chunk::new(compiler::compile(ast));
    timeit!(chunk.disassemble());
    let mut vm = vm::VM::new(chunk, relfilename);
    vm.run();
}

fn compile_file(path: &str, relfilename: &str) {
    let source = fs::read_to_string(path).expect("failed to read file");
    let mut scanner = scanner::Scanner::new(&source, relfilename);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens, relfilename);
    let ast = parser.parse();
    let chunk = compiler::Chunk::new(compiler::compile(ast));
    save_u8(path, chunk.code)
}

fn save_u8(path: &str, bytecode: Vec<OpCode>) {
    let mut save_path = PathBuf::from(path);
    save_path.set_extension("klc");
    //turn the bytecode to Vec<u8>

    let mut file = File::create(save_path).expect("couldnt create file");
    // file.write_all(bytes);
}

fn run_compiled(path: &str, relfilename: &str) {
    println!("running compiled {}", path);
    //decode and run
}
