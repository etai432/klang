#![allow(unused)]
use crate::{error, KlangError};
use std::collections::HashMap;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    pub source: &'a str,
    pub chars: Peekable<Chars<'a>>,
    pub line: usize,
    pub tokens: Vec<Token>,
    filename: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Scanner<'a> {
        Scanner {
            source,
            chars: source.chars().peekable(),
            line: 1,
            tokens: Vec::new(),
            filename,
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
                        while self.chars.next() != Some('\n') && self.chars.peek().is_some() {}
                        self.line += 1;
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
                        error::KlangError::error(
                            KlangError::ScannerError,
                            "missing a second & you fat fuck",
                            self.line,
                            self.filename,
                        )
                    }
                }
                '|' => {
                    if self.is_next('|') {
                        self.make_token(TokenType::Or, ch.to_string(), self.line, None)
                    } else {
                        error::KlangError::error(
                            KlangError::ScannerError,
                            "missing a second | you stupid gay",
                            self.line,
                            self.filename,
                        ) //XD
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
                        self.identifier(ch);
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

    fn identifier(&mut self, ch: char) {
        let mut word = String::from(ch);
        while self.chars.peek().unwrap_or(&'\0').is_ascii_alphabetic() {
            word.push(self.chars.next().unwrap());
        }
        match word.as_str() {
            "let" => self.make_token(TokenType::Let, "".to_string(), self.line, None),
            "in" => self.make_token(TokenType::In, "".to_string(), self.line, None),
            "else" => self.make_token(TokenType::Else, "".to_string(), self.line, None),
            "for" => self.make_token(TokenType::For, "".to_string(), self.line, None),
            "if" => self.make_token(TokenType::If, "".to_string(), self.line, None),
            "print" => self.make_token(TokenType::Print, "".to_string(), self.line, None),
            "while" => self.make_token(TokenType::While, "".to_string(), self.line, None),
            "int" => self.make_token(TokenType::Int, "".to_string(), self.line, None),
            "float" => self.make_token(TokenType::Float, "".to_string(), self.line, None),
            "string" => self.make_token(TokenType::String, "".to_string(), self.line, None),
            "bool" => self.make_token(TokenType::Bool, "".to_string(), self.line, None),
            "true" => self.make_token(
                TokenType::Bool,
                "true".to_string(),
                self.line,
                Some(Value::Bool(true)),
            ),
            "false" => self.make_token(
                TokenType::Bool,
                "false".to_string(),
                self.line,
                Some(Value::Bool(false)),
            ),
            _ => self.make_token(TokenType::Identifier, word, self.line, None),
        }
    }
    fn number(&mut self, ch: char) {
        let mut number = String::from(ch);
        while self.chars.peek().unwrap_or(&'\0').is_ascii_digit() {
            number.push(self.chars.next().unwrap());
        }
        if self.chars.peek().unwrap_or(&'\0') == &'.' {
            number.push(self.chars.next().unwrap());
            if self.chars.peek().unwrap_or(&'\0') == &'.' {
                number.pop();
                self.make_token(TokenType::Int, "".to_string(), self.line,  match number.parse::<i64>() {
                    Ok(e) => Some(Value::Int(e)),
                    Err(_) => panic!("failed to parse integer, fuck you (wouldnt happen if your function works loser)"),
                });
                self.make_token(TokenType::Range, "..".to_string(), self.line, None);
                self.chars.next(); //consume 2nd dot
            } else {
                while self.chars.peek().unwrap_or(&'\0').is_ascii_digit() {
                    number.push(self.chars.next().unwrap());
                }
                if number.ends_with('.') {
                    panic!("float cannot end with a '.'")
                }
                self.make_token(TokenType::Float, "".to_string(), self.line,  match number.parse::<f64>() {
                    Ok(e) => Some(Value::Float(e)),
                    Err(_) => panic!("failed to parse integer, fuck you (wouldnt happen if your function works loser)"),
                })
            }
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
            if self.chars.peek().unwrap_or(&'\0') == &'\n' {
                self.line += 1
            }
            if self.chars.peek().is_none() {
                panic!("Unterminated string");
            }
            string.push(self.chars.next().unwrap());
        }
        self.chars.next(); //consume the 2nd "
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LeftParen"),
            TokenType::RightParen => write!(f, "RightParen"),
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Minus => write!(f, "Minus"),
            TokenType::Plus => write!(f, "Plus"),
            TokenType::Slash => write!(f, "Slash"),
            TokenType::Star => write!(f, "Star"),
            TokenType::Semicolon => write!(f, "Semicolon"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Bang => write!(f, "Bang"),
            TokenType::BangEqual => write!(f, "BangEqual"),
            TokenType::Equal => write!(f, "Equal"),
            TokenType::EqualEqual => write!(f, "EqualEqual"),
            TokenType::Greater => write!(f, "Greater"),
            TokenType::GreaterEqual => write!(f, "GreaterEqual"),
            TokenType::Less => write!(f, "Less"),
            TokenType::LessEqual => write!(f, "LessEqual"),
            TokenType::And => write!(f, "And"),
            TokenType::Or => write!(f, "Or"),
            TokenType::Let => write!(f, "Let"),
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::String => write!(f, "String"),
            TokenType::Int => write!(f, "Int"),
            TokenType::Float => write!(f, "Float"),
            TokenType::Bool => write!(f, "Bool"),
            TokenType::If => write!(f, "If"),
            TokenType::Else => write!(f, "Else"),
            TokenType::For => write!(f, "For"),
            TokenType::Range => write!(f, "Range"),
            TokenType::In => write!(f, "In"),
            TokenType::While => write!(f, "While"),
            TokenType::Print => write!(f, "Print"),
            TokenType::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tt: TokenType,
    pub lexeme: String,
    pub literal: Option<Value>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token: {}, Lexeme: {}, Literal: {:?}, Line: {}",
            self.tt, self.lexeme, self.literal, self.line
        )
    }
}
