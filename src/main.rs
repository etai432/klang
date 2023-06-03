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
use std::time::Instant;
fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: expected file path");
        std::process::exit(1);
    }
    let filename = &args[1];
    let path = Path::new(&filename);
    let relfilename = path.file_name().unwrap().to_str().unwrap();
    if let Err(e) = fs::metadata(filename) {
        eprintln!("Error: {} is not a file: {}", filename, e);
        std::process::exit(1);
    } else {
        run_file(filename, relfilename);
    }

    Ok(())
}

fn run_file(path: &str, relfilename: &str) {
    // let start = Instant::now();
    let source = fs::read_to_string(path).expect("failed to read file");
    let mut scanner = scanner::Scanner::new(&source, relfilename);
    let tokens: Vec<Token> = scanner.scan_tokens();
    let mut parser = parser::Parser::new(tokens, relfilename);
    let ast = parser.parse();
    println!("{:?}\n", ast);
    compiler::Chunk::new(compiler::compile(ast)).disassemble();
    // let duration = start.elapsed();
    // println!(
    //     "Elapsed time: {}.{:03}s",
    //     duration.as_secs(),
    //     duration.subsec_millis()
    // );
}

fn compile_file(path: &str, relfilename: &str) {
    //this function will compile the file to bytecode and save it in the same directory
}
