use types::primary_expression::*;

pub enum Statement<'a> {
    Output(Box<PrimaryExpression<'a>>)
}
