use types::vec_with_cursor::*;
use types::program::*;
use types::function_definition::*;
use types::statement::*;
use types::primary_expression::*;
use types::identifier::*;

type ParserResult<'a, T> = Result<(T, VecWithCursor<'a, char>), VecWithCursor<'a, char>>;

pub fn parse_char<'a>(xs: VecWithCursor<'a, char>, c: char) -> ParserResult<'a, ()> {
    let mut ys = xs;
    let (&y, ys) = ys.next()?;
    if y == c {
        Ok(((), ys))
    }
    else {
        Err(xs)
    }
}

pub fn parse_keyword<'a>(xs: VecWithCursor<'a, char>, ss: &str) -> ParserResult<'a, ()> {
    let mut ys = xs;
    let (_, ys) = parse_whitespaces(ys)?;
    for c in ss.chars() {
        let (_, ys2) = parse_char(ys, c)?;
        ys = ys2;
    }
    let (_, ys) = parse_whitespace(ys)?;
    Ok(((), ys))
}

pub fn parse_delimiter<'a>(xs: VecWithCursor<'a, char>, c: char) -> ParserResult<'a, ()> {
    assert!(c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}' || c == ';' || c == ',');
    let mut ys = xs;
    let (_, ys) = parse_whitespaces(ys)?;
    let (_, ys) = parse_char(ys, c)?;
    Ok(((), ys))
}

pub fn parse_whitespace<'a>(xs: VecWithCursor<'a, char>) -> ParserResult<'a, ()> {
    parse_char(xs, ' ')
        .or_else(|_| parse_char(xs, '\n'))
        .or_else(|_| parse_char(xs, '\t'))
}

pub fn parse_whitespaces<'a>(xs: VecWithCursor<'a, char>) -> ParserResult<'a, ()> {
    let mut ys = xs;
    while let Ok(((), ys2)) = parse_whitespace(ys) {
        ys = ys2;
    }
    Ok(((), ys))
}

pub fn parse_identifier<'a>(xs: VecWithCursor<'a, char>) -> ParserResult<'a, Identifier<'a>> {
    let mut ys = xs;
    let (_, ys) = parse_whitespaces(ys)?;
    let (&y, _) = ys.next()?;
    if y.is_alphabetic() || y == '_' {
        let (ident, ys) = ys.take_while(|&c| c.is_alphanumeric() || y == '_');
        Ok((Identifier { ss: ident }, ys))
    }
    else {
        Err(ys)
    }
}

pub fn parse_statement<'a>(xs: VecWithCursor<'a, char>) -> ParserResult<'a, Statement<'a>> {
    unimplemented!();
}

pub fn parse_function_definition<'a>(xs: VecWithCursor<'a, char>) -> ParserResult<'a, FunctionDefinition<'a>> {
    let mut ys = xs;
    let (_, ys) = parse_keyword(ys, "fn")?;
    let (name, ys) = parse_identifier(ys)?;
    let (_, ys) = parse_delimiter(ys, '(')?;
    let (_, ys) = parse_delimiter(ys, ')')?;
    let (_, ys) = parse_delimiter(ys, '{')?;
    let mut statements = vec![];
    while let Ok((statement, ys2)) = parse_statement(ys) {
        ys = ys2;
        statements.push(statement);
    }
    let (_, ys) = parse_delimiter(ys, '}')?;
    Ok((FunctionDefinition::<'a> {
        name,
        statements,
    }, ys))
}

pub fn parse_program<'a>(xs: VecWithCursor<'a, char>) -> ParserResult<'a, Program<'a>> {
    let mut function_definitions = vec![];
    let mut ys = xs;
    while let Ok((function_definition, ys2)) = parse_function_definition(ys) {
        ys = ys2;
        function_definitions.push(function_definition);
    }
    Ok((Program::<'a> {
        function_definitions: function_definitions,
    }, ys))
}
