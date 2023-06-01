use crate::error::KlangError;
use crate::expr::Expr;
use crate::scanner::{Token, TokenType, Value};

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
    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.range()
    }

    pub fn range(&mut self) -> Expr {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Identifier]) {
            if self.match_tokens(&[TokenType::Equal]) {
                let name: Token = self.previous();
                let value: Expr = self.logical();
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                };
            }
            self.error("missing = sign u fuckin bozo")
        }
        self.logical()
    }

    pub fn logical(&mut self) -> Expr {
        let left: Expr = self.equality();
        if self.match_tokens(&[TokenType::And, TokenType::Or]) {
            let operator = self.previous();
            let right: Expr = self.equality();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }

    fn equality(&mut self) -> Expr {
        let left: Expr = self.comparison();
        if self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right: Expr = self.comparison();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
    fn comparison(&mut self) -> Expr {
        let left: Expr = self.term();
        if self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right: Expr = self.term();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
    fn term(&mut self) -> Expr {
        let left: Expr = self.factor();
        if self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous();
            let right: Expr = self.factor();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
    fn factor(&mut self) -> Expr {
        let left: Expr = self.unary();
        if self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right: Expr = self.unary();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let e = self.unary();
            return Expr::Unary {
                operator,
                expression: Box::new(e),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bool]) {
            if self.previous().lexeme == "true" {
                return Expr::Literal(Value::Bool(true));
            } else {
                return Expr::Literal(Value::Bool(false));
            }
        }
        if self.match_tokens(&[TokenType::String]) {
            let string = self.previous().lexeme;
            let mut printables: Vec<Token> = Vec::new();
            while self.match_tokens(&[TokenType::Printable]) {
                printables.push(self.previous())
            }
            return Expr::Literal(Value::String { string, printables });
        }

        if self.match_tokens(&[TokenType::Int, TokenType::Float]) {
            return Expr::Literal(self.previous().literal.unwrap());
        }

        /* if self.match_tokens(&[TokenType::Float]) {
                   return Expr::Literal(self.previous().literal.unwrap());
               }
        */
        if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.or();
            self.consume(
                TokenType::RightParen,
                "expected \")\" after expression u piece of shit",
            );
            return Expr::Grouping {
                expression: Box::new(expression),
            };
        }

        if self.match_tokens(&[TokenType::Identifier]) {
            return Expr::Variable {
                name: self.previous(),
            };
        }

        self.error("expected expression 8===D");
        panic!("cock!")
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        // checks if next token is in types slice thing

        for &tt in types {
            if self.check(tt) {
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
