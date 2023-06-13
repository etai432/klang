use error::KlangError;
use std::path::Path;
use std::{env, fs};
mod compiling;
use compiling::{compiler, vm};
mod error;
mod interpreter;
use interpreter::{parser, scanner};

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
    if args.len() >= 2 {
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
}

fn run_file(path: &str, relfilename: &str) {
    let source = fs::read_to_string(path).expect("failed to read file");
    let mut scanner = scanner::Scanner::new(&source, relfilename);
    let mut parser = parser::Parser::new(scanner.scan_tokens(), relfilename);
    let ast = parser.parse();
    let chunk = compiler::Chunk::new(compiler::compile(ast));
    let mut vm = vm::VM::new(chunk, relfilename);
    vm.run();
}
