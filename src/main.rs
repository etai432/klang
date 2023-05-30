use std::{env, fs};
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: expected file path");
        std::process::exit(1);
    }
    let filename = &args[1];
    if let Err(e) = fs::metadata(filename) {
        eprintln!("Error: {} is not a file: {}", filename, e);
        std::process::exit(1);
    } else {
        run_file(filename);
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("failed to read file");
    scanner::Scanner::new(&source);
    println!("{}", source);
}
