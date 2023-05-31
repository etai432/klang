use crate::error::KlangError;
use crate::expr::Expr;
use crate::scanner::{Token, TokenType};

pub struct Parser<'a> {
    pub tokens: Vec<Token>,
    current: usize,
    filename: &'a str,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, filename: &'a str) -> Parser {
        Parser {
            tokens,
            current: 0,
            filename,
        }
    }
    fn equality(&mut self) -> Expr {
        let left: Expr = self.comparison();
        if self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.advance();
            let right: Expr = self.comparison();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
    fn comparison(&mut self) -> Expr {}
    fn term(&mut self) -> Expr {}
    fn factor(&mut self) -> Expr {}
    fn unary(&mut self) -> Expr {}
    fn primary(&mut self) -> Expr {
        //handle bool (true, false)
        //handle string
        //handle int
        //handle float
        //return Literal
        //also handle for grouping and variable
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        // checks if next token is in types slice thing

        for &i in types {
            if self.check(i) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, t_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tt == t_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn is_at_end(&self) -> bool {
        self.peek().tt == TokenType::Eof
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn error(&self, msg: &str) {
        KlangError::error(
            KlangError::ParserError,
            msg,
            self.peek().line,
            self.filename,
        );
    }
    fn consume(&mut self, t_type: TokenType, msg: &str) -> Token {
        if self.peek().tt == t_type {
            return self.advance();
        }
        self.error(msg);
        panic!()
    }
}
