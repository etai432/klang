use std::{env, fs};
mod error;
mod expr;
mod parser;
mod scanner;
use error::KlangError;
use scanner::Token;
mod compiler;
mod opcode;
mod stmt;
mod vm;
use std::path::Path;

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
        eprintln!("Error: expected file path");
        std::process::exit(1);
    }
    let filename = &args[1];

    let path = Path::new(&filename);
    let relfilename = path.file_name().unwrap().to_str().unwrap();
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

fn run_file(path: &str, relfilename: &str) {
    let source = fs::read_to_string(path).expect("failed to read file");
    let mut scanner = scanner::Scanner::new(&source, relfilename);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens, relfilename);
    let ast = parser.parse();
    println!("{:?}\n", ast);
    timeit!(compiler::Chunk::new(compiler::compile(ast)).disassemble());
}

// fn compile_file(path: &str, relfilename: &str) {
//     //this function will compile the file to bytecode and save it in the same directory
// }
