extern crate meta_text;

use std::fs;
use std::io::{Read};

use meta_text::entry::*;
use meta_text::types::value::*;
use meta_text::lexer::lex;
use meta_text::eval::*;
use meta_text::parser::expression::*;
use meta_text::render::render;

#[test]
fn test_1() {
    entry();
}

fn read_file(filename: &str) -> String {
    let mut file = fs::File::open(filename).unwrap();
    let mut ss = String::new();
    file.read_to_string(&mut ss).unwrap();
    ss
}

#[test]
fn test_read_file() {
    let ss = read_file("tests/testcases/001-in.mt");
    assert_eq!(ss, "\"abcd\\n\";\n");
}

#[test]
fn test_4() {
    for i in 1..10+1 {
        let input_filename = format!("tests/testcases/{: >03}-in.mt", i);
        let input = read_file(input_filename.as_str());
        let exp_output_filename = format!("tests/testcases/{: >03}-out.txt", i);
        let exp_output = read_file(exp_output_filename.as_str());
        let output = render(input);
        assert_eq!(exp_output, output);
    }
}

#[test]
fn test_eval_expression() {
    let ss: Vec<char> = "1 + 2 * 3".to_string().chars().collect();
    let ts = lex(&ss);
    let table = gen_default_precedence_table();
    let mut p = ts.iter().peekable();
    let exp = parse_expression(&mut p, &table);
    let t = eval_expression(&exp);
    eprintln!("{:?}", &exp);
    assert_eq!(t, Value::Num(7));
}
