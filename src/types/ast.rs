use types::token::*;
use types::expression::*;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum AST {
    Seq(Vec<Box<AST>>),
    String(Token),
    Expression(Box<Expression>),
    Empty
}

impl AST {
    pub fn new() -> AST {
        AST::Empty
    }
}
