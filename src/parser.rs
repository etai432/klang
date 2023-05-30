use crate::expr::Expr;
use crate::scanner::{Token, TokenType};
pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }
    fn equality(&mut self) -> Expr {
        let left: Expr = self.comparison();
        if self.previous() != "==" || self.previous() != "!=" {
            return left;
        } else {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            Expr::Binary {
                left,
                operator,
                right,
            }
        }
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
        //checks if the next token is one of the types
    }
    fn check(&self, t_type: TokenType) -> bool {
        //check if the next token is t_type
    }
    fn advance(&mut self) -> Token {
        //advances by 1
    }
    fn is_at_end(&self) -> bool {
        self.peek().t_type == TokenType::Eof
    }
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn error(&self) {
        //calls parser error
    }
    fn consume(&mut self, t_type: TokenType, msg: &str) -> Option<Token> {
        //checking if the next token is of type t_type, and returning it, or erroring.
    }
}
