use std::collections::BTreeMap;
use std::slice::Iter;
use std::iter::Peekable;

use types::token::*;
use types::expression::*;
use types::precedence::*;

use parser::expression::*;

pub fn parse_atom(iter: &mut Peekable<Iter<Token>>, precedence_table: &BTreeMap<Vec<char>, Precedence>) -> Expression {
    if let Some(head) = iter.peek() {
        match head {
            Token::NUM(_) => {
            }
            Token::DELIMITER(_) => {
            }
            _ => {
                panic!();
            }
        }
    }
    else {
        panic!();
    }
    if let Some(head) = iter.next() {
        match head {
            Token::NUM(_) => {
                Expression::Literal(head.clone())
            }
            Token::DELIMITER(d) => {
                if *d == vec!['('] {
                    let res = parse_expression(iter, precedence_table);
                    iter.next();
                    res
                }
                else {
                    panic!();
                }
            }
            _ => {
                panic!();
            }
        }
    }
    else {
        panic!();
    }
}
