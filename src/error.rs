#![allow(dead_code)]

use colored::*;
#[derive(Debug)]
pub enum KlangError {
    ScannerError,
    ParserError,
    RuntimeError,
}

impl KlangError {
    pub fn error(et: KlangError, msg: &str, line: usize, filename: &str) {
        eprintln!(
            "{}",
            format!("[{et:?}] {filename} at line {line}: {msg}").red()
        );
        std::process::exit(0);
    }
}
