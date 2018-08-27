use std::collections::BTreeMap;
use std::slice::Iter;
use std::iter::Peekable;

use types::token::*;
use types::expression::*;
use types::precedence::*;

use parser::atom::*;

pub fn parse_expression(iter: &mut Peekable<Iter<Token>>, precedence_table: &BTreeMap<Vec<char>,Precedence>) -> Expression {
    parse_expression_sub(iter, precedence_table, None)
}

pub fn parse_expression_sub(iter: &mut Peekable<Iter<Token>>, precedence_table: &BTreeMap<Vec<char>,Precedence>, prev_prec: Option<Precedence>) -> Expression {
    let mut res = parse_atom(iter, precedence_table);
    loop {
        if let Some(head) = iter.peek() {
            match head {
                Token::OP(ss) => {
                    eprintln!("ss = {:?}", ss);
                    let prec = precedence_table[ss];
                    if prev_prec.is_none() || prev_prec.unwrap().cont(&prec) {
                    }
                    else {
                        break;
                    }
                }
                Token::DELIMITER(_) => {
                    break;
                }
                _ => {
                    eprintln!("{:?}", head);
                    panic!();
                }
            }
        }
        else {
            break;
        }
        if let Some(head) = iter.next() {
            match head {
                Token::OP(ss) => {
                    let prec = precedence_table[ss];
                    let right = parse_expression_sub(iter, precedence_table, Some(prec));
                    res = Expression::BinaryOp(Box::new(res), head.clone(), Box::new(right));
                }
                _ => {
                    panic!();
                }
            }
        }
    }
    res
}
