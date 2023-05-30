use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub chars: Peekable<Chars<'a>>,
    pub line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        let mut keywords = HashMap::new();
        keywords.insert("let", TokenType::Let);
        keywords.insert("in", TokenType::In);
        keywords.insert("else", TokenType::Else);
        keywords.insert("for", TokenType::For);
        keywords.insert("if", TokenType::If);
        keywords.insert("print", TokenType::Print);
        keywords.insert("while", TokenType::While);
        keywords.insert("int", TokenType::Int);
        keywords.insert("float", TokenType::Float);
        keywords.insert("string", TokenType::String);
        keywords.insert("bool", TokenType::Bool);
        Scanner {
            source,
            chars: source.chars().peekable(),
            line: 1,
            keywords,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Semicolon,
    Colon,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Let,
    Identifier,
    String,
    Int,
    Float,
    Bool,
    If,
    Else,
    For,
    Range,
    In,
    While,
    Print,
    Eof,
}
