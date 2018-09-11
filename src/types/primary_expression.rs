use types::expression::*;

pub enum PrimaryExpression<'a> {
    Identifier(&'a [char]),
    NumericalValue(&'a [char]),
    Character(&'a [char]),
    String {
        strings: Vec<&'a [char]>,
        exps: Vec<Box<Expression<'a>>>
    },
    UniOP {
        op: &'a [char],
        exp: Box<PrimaryExpression<'a>>
    }
}
