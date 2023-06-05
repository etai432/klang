#![allow(unused)]
use std::io::BufRead;

use crate::error::KlangError;
use crate::expr::Expr;
use crate::scanner::Scanner;
use crate::scanner::{Token, TokenType, Value};
use crate::stmt::Stmt;

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
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }
    fn declaration(&mut self) -> Stmt {
        if self.match_tokens(&[TokenType::Let]) {
            self.var_decl()
        } else if self.match_tokens(&[TokenType::Fn]) {
            self.fn_decl()
        } else {
            self.statement()
        }
    }
    fn declaration_inner(&mut self) -> Stmt {
        if self.match_tokens(&[TokenType::Let]) {
            self.var_decl()
        } else {
            self.statement()
        }
    }

    fn fn_decl(&mut self) -> Stmt {
        if self.match_tokens(&[
            TokenType::Int,
            TokenType::Float,
            TokenType::Bool,
            TokenType::String,
        ]) {
            let return_t = self.previous();
            let name = self.consume(TokenType::Identifier, "must have a function name");
            if self.match_tokens(&[TokenType::LeftParen]) {
                if self.match_tokens(&[TokenType::RightParen]) {
                    return Stmt::Fn {
                        return_t: Some(return_t),
                        name,
                        params: Vec::new(),
                        body: Box::new(self.block()),
                    };
                }
                let mut vec: Vec<(Token, Token)> = Vec::new();
                let mut iden =
                    self.consume(TokenType::Identifier, "argument must be an identifier");
                self.consume(TokenType::Colon, "must specify identifier type using a :");
                let mut t: Token;
                if self.match_tokens(&[
                    TokenType::Int,
                    TokenType::Float,
                    TokenType::Bool,
                    TokenType::String,
                ]) {
                    t = self.previous();
                } else {
                    self.error("must specify identifier type");
                    panic!("cock")
                }
                vec.push((iden, t));
                while self.match_tokens(&[TokenType::Comma]) {
                    iden = self.consume(TokenType::Identifier, "parameter must be an identifier");
                    self.consume(TokenType::Colon, "must specify identifier type using a :");
                    if self.match_tokens(&[
                        TokenType::Int,
                        TokenType::Float,
                        TokenType::Bool,
                        TokenType::String,
                    ]) {
                        t = self.previous();
                    } else {
                        self.error("must specify identifier type");
                        panic!("cock")
                    }
                    vec.push((iden, t));
                }
                self.consume(TokenType::RightParen, "gotta close the call dude");
                return Stmt::Fn {
                    return_t: Some(return_t),
                    name,
                    params: vec,
                    body: Box::new(self.block()),
                };
            }
        } else {
            let name = self.consume(TokenType::Identifier, "must have a function name");
            if self.match_tokens(&[TokenType::LeftParen]) {
                if self.match_tokens(&[TokenType::RightParen]) {
                    return Stmt::Fn {
                        return_t: None,
                        name,
                        params: Vec::new(),
                        body: Box::new(self.block()),
                    };
                }
                let mut vec: Vec<(Token, Token)> = Vec::new();
                let mut iden =
                    self.consume(TokenType::Identifier, "argument must be an identifier");
                self.consume(TokenType::Colon, "must specify identifier type using a :");
                let mut t: Token;
                if self.match_tokens(&[
                    TokenType::Int,
                    TokenType::Float,
                    TokenType::Bool,
                    TokenType::String,
                ]) {
                    t = self.previous();
                } else {
                    self.error("must specify identifier type");
                    panic!("cock")
                }
                vec.push((iden, t));
                while self.match_tokens(&[TokenType::Comma]) {
                    iden = self.consume(TokenType::Identifier, "parameter must be an identifier");
                    self.consume(TokenType::Colon, "must specify identifier type using a :");
                    if self.match_tokens(&[
                        TokenType::Int,
                        TokenType::Float,
                        TokenType::Bool,
                        TokenType::String,
                    ]) {
                        t = self.previous();
                    } else {
                        self.error("must specify identifier type");
                        panic!("cock")
                    }
                    vec.push((iden, t));
                }
                self.consume(TokenType::RightParen, "gotta close the call dude");
                return Stmt::Fn {
                    return_t: None,
                    name,
                    params: vec,
                    body: Box::new(self.block()),
                };
            }
        }
        panic!("if you get here, ksang has problemo in code")
    }
    fn var_decl(&mut self) -> Stmt {
        let name = self.consume(TokenType::Identifier, "must define a variable name");
        self.consume(TokenType::Colon, "must specify variable type using a :");
        if self.match_tokens(&[
            TokenType::Int,
            TokenType::Float,
            TokenType::Bool,
            TokenType::String,
        ]) {
            let t = self.previous();
            if self.match_tokens(&[TokenType::Equal]) {
                let value = self.logical();
                self.consume(TokenType::Semicolon, "missing ; at the end of the line");
                return Stmt::Var {
                    name,
                    t,
                    value: Some(value),
                };
            }
            self.consume(TokenType::Semicolon, "missing ; at the end of the line");
            return Stmt::Var {
                name,
                t,
                value: None,
            };
        }
        self.error("must specify variable type");
        panic!()
    }

    fn statement(&mut self) -> Stmt {
        if self.match_tokens(&[TokenType::Print]) {
            self.print_stmt()
        } else if self.check(TokenType::LeftBrace) {
            self.block()
        } else if self.match_tokens(&[TokenType::If]) {
            self.if_stmt()
        } else if self.match_tokens(&[TokenType::While]) {
            self.while_stmt()
        } else if self.match_tokens(&[TokenType::For]) {
            self.for_stmt()
        } else if self.match_tokens(&[TokenType::Return]) {
            self.return_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn return_stmt(&mut self) -> Stmt {
        let value = self.logical();
        self.consume(TokenType::Semicolon, "missing ; at the end of lien");
        Stmt::Return(value, self.previous().line)
    }

    fn for_stmt(&mut self) -> Stmt {
        let identifier = self.consume(TokenType::Identifier, "missing identifier 8=D");
        let line = self.previous().line;
        self.consume(TokenType::In, "missing in");
        let iterable = self.range();
        match iterable {
            Expr::Range {
                min: _,
                max: _,
                step: _,
                line: _,
            } => (),
            _ => self.error("\"in\" must be used on an iterable"),
        }

        let block = Box::new(self.block());
        Stmt::For {
            identifier,
            iterable,
            block,
            line,
        }
    }

    fn if_stmt(&mut self) -> Stmt {
        let condition = self.logical();
        let start = self.previous().line;
        let block = Box::new(self.block());
        if self.match_tokens(&[TokenType::Else]) {
            let end = self.previous().line;
            let elseblock = Some(Box::new(self.block()));
            return Stmt::If {
                condition,
                block,
                elseblock,
                lines: (start, Some(end)),
            };
        }
        Stmt::If {
            condition,
            block,
            elseblock: None,
            lines: (start, None),
        }
    }

    fn while_stmt(&mut self) -> Stmt {
        let condition = self.logical();
        let line = self.previous().line;
        let block = self.block();

        Stmt::While {
            condition,
            block: Box::new(block),
            line,
        }
    }

    fn block(&mut self) -> Stmt {
        self.consume(TokenType::LeftBrace, "must start block with a {");
        let start = self.previous().line;
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() && !self.check(TokenType::RightBrace) {
            statements.push(self.declaration_inner());
        }
        self.consume(TokenType::RightBrace, "must end block with a }");
        Stmt::Block(statements, (start, self.previous().line))
    }

    fn print_stmt(&mut self) -> Stmt {
        self.consume(
            TokenType::LeftParen,
            "gotta put ( after a print yk how it is..",
        );
        let stmt = Stmt::Print(
            match self.primary() {
                Expr::Literal(Value::String { string, printables }, _) => {
                    Value::String { string, printables }
                }
                _ => {
                    self.error("can only print strings");
                    panic!("balls")
                }
            },
            self.peek().line,
        );
        self.consume(
            TokenType::RightParen,
            "gotta put ) at the end of a print yk how it is..",
        );
        self.consume(TokenType::Semicolon, "missing ; at the end of the line");
        stmt
    }
    fn expr_stmt(&mut self) -> Stmt {
        let stmt = Stmt::Expression(self.assignment());
        self.consume(TokenType::Semicolon, "missing ; at the end of the line");
        stmt
    }

    pub fn assignment(&mut self) -> Expr {
        let identifier = self.logical();
        if self.match_tokens(&[TokenType::Equal]) {
            let value = self.logical();
            match identifier {
                Expr::Variable(name) => {
                    return Expr::Assign {
                        name,
                        value: Box::new(value),
                    }
                }
                _ => {
                    self.error("cannot assign to a non variable");
                    panic!();
                }
            }
        }
        identifier
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
            let right: Expr = self.term();
            return Expr::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }
    fn factor(&mut self) -> Expr {
        let left: Expr = self.range();
        if self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
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
    pub fn range(&mut self) -> Expr {
        let start = self.unary();
        if self.match_tokens(&[TokenType::Range]) {
            match &start {
                Expr::Literal(Value::Int(_), _) | Expr::Variable(_) => {}
                _ => {
                    self.error("you can only index a range using an integer");
                    panic!();
                }
            }
            let end = self.unary();
            match &end {
                Expr::Literal(Value::Int(_), _) | Expr::Variable(_) => {}
                _ => {
                    self.error("you can only index a range using an integer");
                    panic!();
                }
            }
            if self.match_tokens(&[TokenType::Range]) {
                let step = self.unary();
                match &step {
                    Expr::Literal(Value::Int(_), _) | Expr::Variable(_) => {}
                    _ => {
                        self.error("you can only index a range using an integer");
                        panic!();
                    }
                }
                return Expr::Range {
                    min: Box::new(start),
                    max: Box::new(end),
                    step: Some(Box::new(step)),
                    line: self.previous().line,
                };
            }
            return Expr::Range {
                min: Box::new(start),
                max: Box::new(end),
                step: None,
                line: self.previous().line,
            };
        }
        start
    }
    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let e = self.primary();
            return Expr::Unary {
                operator,
                expression: Box::new(e),
            };
        }
        self.call()
    }

    fn call(&mut self) -> Expr {
        let expr = self.primary();
        if self.match_tokens(&[TokenType::LeftParen]) {
            if !matches!(expr, Expr::Variable(_)) {
                self.error("sir were you trying to call a function USING AN INTEGER?")
            }
            if self.match_tokens(&[TokenType::RightParen]) {
                return Expr::Call {
                    callee: Box::new(expr),
                    arguments: Vec::new(),
                };
            }
            let mut vec: Vec<Expr> = Vec::new();
            vec.push(self.logical());
            while self.match_tokens(&[TokenType::Comma]) {
                vec.push(self.logical());
            }
            self.consume(TokenType::RightParen, "gotta close the call dude");
            return Expr::Call {
                callee: Box::new(expr),
                arguments: vec,
            };
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bool]) {
            if self.previous().lexeme == "true" {
                return Expr::Literal(Value::Bool(true), self.previous().line);
            } else {
                return Expr::Literal(Value::Bool(false), self.previous().line);
            }
        }
        if self.match_tokens(&[TokenType::String]) {
            let string = self.previous().lexeme;
            let mut printables_t: Vec<Vec<Token>> = Vec::new();
            while self.match_tokens(&[TokenType::Printable]) {
                let lexeme = self.previous().lexeme;
                let mut s = Scanner::new(&lexeme, self.filename);
                let mut s1 = s.scan_tokens();
                s1.pop();
                printables_t.push(s1);
                self.match_tokens(&[TokenType::Comma]);
            }
            let mut printables: Vec<Expr> = Vec::new();
            for i in printables_t {
                self.tokens.splice(self.current..self.current, i);
                printables.push(self.logical());
            }
            return Expr::Literal(Value::String { string, printables }, self.previous().line);
        }

        if self.match_tokens(&[TokenType::Int, TokenType::Float]) {
            return Expr::Literal(self.previous().literal.unwrap(), self.previous().line);
        }
        if self.match_tokens(&[TokenType::LeftParen]) {
            let expression = self.logical();
            self.consume(
                TokenType::RightParen,
                "expected \")\" after expression u piece of shit",
            );
            return Expr::Grouping(Box::new(expression));
        }

        if self.match_tokens(&[TokenType::Identifier]) {
            return Expr::Variable(self.previous());
        }
        self.error(&format!("expected value found {}", self.peek().tt));
        panic!("cock!")
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
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
