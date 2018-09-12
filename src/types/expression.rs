use types::primary_expression::*;

pub enum Expression<'a> {
    PrimaryExpression(Box<PrimaryExpression<'a>>),
    BinOp {
        left: Box<Expression<'a>>,
        op: &'a [char],
        right: Box<Expression<'a>>
    }
}
