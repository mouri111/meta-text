use types::token::*;

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Expression {
    BinaryOp(Box<Expression>,Token,Box<Expression>),
    UnaryOp(Token,Box<Expression>),
    Literal(Token)
}
