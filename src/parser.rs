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
}
