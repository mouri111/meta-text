use std::collections::BTreeMap;

use types::ast::*;
use types::precedence::*;
use types::token::*;
use types::value::*;

use lexer::lex;
use parser::expression::parse_expression;
use parser::parse;

use eval::*;

pub fn render_dfs(ast: &AST, buf: &mut String, prec_table: &mut BTreeMap<Vec<char>, Precedence>) {
    match ast {
        AST::Seq(xs) => {
            for x in xs.iter() {
                render_dfs(x, buf, prec_table);
            }
        }
        AST::String(token) => {
            match token {
                Token::STRING(ss) => {
                    let n = ss.len();
                    let mut exp_buf: Vec<char> = vec![];
                    let mut exp_mode = false;
                    for i in 1..n-1 {
                        if ss[i] == '{' {
                            exp_mode = true;
                        }
                        else if ss[i] == '}' {
                            let ts = lex(&exp_buf);
                            let mut p = ts.iter().peekable();
                            let exp = parse_expression(&mut p, &prec_table);
                            let t = eval_expression(&exp);
                            match t {
                                Value::Num(n) => {
                                    for c in n.to_string().chars() {
                                        buf.push(c);
                                    }
                                }
                            }
                            exp_buf.clear();
                            exp_mode = false;
                        }
                        else if exp_mode {
                            exp_buf.push(ss[i]);
                        }
                        else {
                            buf.push(ss[i]);
                        }
                    }
                }
                _ => {
                }
            }
        }
        AST::Expression(_) => {
        }
        AST::Empty => {
        }
    }
}

pub fn render(ss: String) -> String {
    let ss = ss.chars().collect();
    let ts = lex(&ss);
    let ast = parse(ts);
    eprintln!("{:?}", ast);
    let mut buf = String::new();
    let mut prec_table = gen_default_precedence_table();
    render_dfs(&ast, &mut buf, &mut prec_table);
    buf
}
