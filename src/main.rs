use std::{env, fs};
mod error;
mod scanner;
use error::KlangError;
use std::path::Path;

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
    let source = fs::read_to_string(path).expect("failed to read file");
    let mut scanner = scanner::Scanner::new(&source, relfilename);
    scanner.scan_tokens();
    println!("{:?}", scanner.tokens);
}
