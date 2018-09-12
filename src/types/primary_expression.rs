use types::expression::*;
use types::identifier::*;

pub enum PrimaryExpression<'a> {
    Identifier(Identifier<'a>),
    NumericalValue(&'a [char]),
    Character(&'a [char]),
    String {
        strings: Vec<&'a [char]>,
        exps: Vec<Expression<'a>>
    },
    UniOP {
        op: &'a [char],
        exp: Box<PrimaryExpression<'a>>
    }
}
