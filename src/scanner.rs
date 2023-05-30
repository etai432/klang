#![allow(unused)]
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
                '!' => {
                    if self.is_next('=') {
                        self.make_token(TokenType::BangEqual, ch.to_string(), self.line, None)
                    } else {
                        self.make_token(TokenType::Bang, ch.to_string(), self.line, None)
                    }
                }
                '=' => {
                    if self.is_next('=') {
                        self.make_token(TokenType::EqualEqual, ch.to_string(), self.line, None)
                    } else {
                        self.make_token(TokenType::Equal, ch.to_string(), self.line, None)
                    }
                }
                '>' => {
                    if self.is_next('=') {
                        self.make_token(TokenType::GreaterEqual, ch.to_string(), self.line, None)
                    } else {
                        self.make_token(TokenType::Greater, ch.to_string(), self.line, None)
                    }
                }
                '<' => {
                    if self.is_next('=') {
                        self.make_token(TokenType::LessEqual, ch.to_string(), self.line, None)
                    } else {
                        self.make_token(TokenType::Less, ch.to_string(), self.line, None)
                    }
                }
                '.' => {
                    if self.is_next('.') {
                        self.make_token(TokenType::Range, ch.to_string(), self.line, None)
                    } else {
                        self.make_token(TokenType::Dot, ch.to_string(), self.line, None)
                    }
                }
                '&' => {
                    if self.is_next('&') {
                        self.make_token(TokenType::And, ch.to_string(), self.line, None)
                    } else {
                        panic!("missing second & u fucking FUCKER") //XD
                    }
                }
                '|' => {
                    if self.is_next('|') {
                        self.make_token(TokenType::Or, ch.to_string(), self.line, None)
                    } else {
                        panic!("missing second | u fucking FUCKER") //XD
                    }
                }
                '"' => self.string(),
                ' ' => (),
                '\r' => (),
                '\t' => (),
                '\n' => self.line += 1,
                _ => {
                    if ch.is_ascii_digit() {
                        self.number(ch);
                    } else if ch.is_ascii_alphabetic() {
                        self.identifier();
                    } else {
                        panic!("unexpected character");
                    }
                }
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
    fn identifier(&mut self) {
        //check for keyword first
    }
    fn number(&mut self, ch: char) {
        let mut number = String::from(ch);
        while self.chars.peek().unwrap_or(&'\0').is_ascii_digit() {
            number.push(self.chars.next().unwrap());
        }
        if self.chars.peek().unwrap_or(&'\0') == &'.' {
            number.push(self.chars.next().unwrap());
            while self.chars.peek().unwrap_or(&'\0').is_ascii_digit() {
                number.push(self.chars.next().unwrap());
            }
            if number.ends_with('.') {
                panic!("float cannot end with a '.'")
            }
            self.make_token(TokenType::Int, "".to_string(), self.line,  match number.parse::<f64>() {
                Ok(e) => Some(Value::Float(e)),
                Err(_) => panic!("failed to parse integer, fuck you (wouldnt happen if your function works loser)"),
            })
        } else {
            self.make_token(TokenType::Int, "".to_string(), self.line,  match number.parse::<i64>() {
                Ok(e) => Some(Value::Int(e)),
                Err(_) => panic!("failed to parse integer, fuck you (wouldnt happen if your function works loser)"),
            })
        }
    }
    fn string(&mut self) {
        let mut string = String::new();
        while self.chars.peek().unwrap_or(&'\0') != &'"' {
            if self.chars.peek().unwrap() == &'\n' {
                self.line += 1
            }
            string.push(self.chars.next().unwrap());
        }

        self.make_token(TokenType::String, format!("\"{string}\""), self.line, None)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
