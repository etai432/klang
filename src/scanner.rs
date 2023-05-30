use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub chars: Peekable<Chars<'a>>,
    pub line: usize,
    pub tokens: Vec<Token>,
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
            tokens: Vec::new(),
            keywords,
        }
    }
    pub fn scan_tokens(&mut self) {
        while let Some(ch) = self.chars.next() {
            match ch {
                '(' => self.make_token(TokenType::LeftParen, ch.to_string(), self.line, None),
                ')' => self.make_token(TokenType::RightParen, ch.to_string(), self.line, None),
                '{' => self.make_token(TokenType::LeftBrace, ch.to_string(), self.line, None),
                '}' => self.make_token(TokenType::RightBrace, ch.to_string(), self.line, None),
                ',' => self.make_token(TokenType::Comma, ch.to_string(), self.line, None),
                '-' => self.make_token(TokenType::Minus, ch.to_string(), self.line, None),
                '+' => self.make_token(TokenType::Plus, ch.to_string(), self.line, None),
                ';' => self.make_token(TokenType::Semicolon, ch.to_string(), self.line, None),
                '*' => self.make_token(TokenType::Star, ch.to_string(), self.line, None),
                ':' => self.make_token(TokenType::Colon, ch.to_string(), self.line, None),
                '/' => {
                    if self.is_next('/') {
                        while self.chars.next() != Some('\n') || self.chars.peek().is_none() {}
                    } else {
                        self.make_token(TokenType::Slash, ch.to_string(), self.line, None);
                    }
                }
                _ => todo!("multi character tokens go here"),
            }
        }
        self.make_token(TokenType::Eof, String::from(""), self.line, None);
    }
    fn make_token(&mut self, tt: TokenType, text: String, line: usize, value: Option<Value>) {
        self.tokens.push(Token {
            tt,
            lexeme: text,
            literal: value,
            line,
        })
    }
    fn is_next(&mut self, ch: char) -> bool {
        self.chars.peek() == Some(&ch)
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    Value(Value),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme: String,
    pub literal: Option<Value>,
    pub line: usize,
}
