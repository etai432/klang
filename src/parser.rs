use crate::expr::Expr;
pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }
    fn equality() -> Expr {}
    fn comparison() -> Expr {}
    fn term() -> Expr {}
    fn factor() -> Expr {}
    fn unary() -> Expr {}
    fn primary() -> Expr {}

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
}
