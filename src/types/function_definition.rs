use types::statement::*;

pub struct FunctionDefinition<'a> {
    name: &'a [char],
    statements: Vec<Statement<'a>>
}
