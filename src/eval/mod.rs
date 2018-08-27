use std::collections::BTreeMap;

use types::token::*;
use types::value::*;
use types::expression::*;
use types::precedence::*;

pub fn eval_literal(token: &Token) -> Value {
    match token {
        Token::NUM(ss) => {
            let mut t = 0i64;
            for &c in ss {
                t = t * 10 + (c as i64 - '0' as i64) as i64;
            }
            Value::Num(t)
        }
        _ => {
            panic!();
        }
    }
}

pub fn eval_expression(exp: &Expression) -> Value {
    match exp {
        Expression::BinaryOp(left, op, right) => {
            if *op == Token::OP(vec!['+']) {
                match eval_expression(left) {
                    Value::Num(l) => {
                        match eval_expression(right) {
                            Value::Num(r) => {
                                Value::Num(l + r)
                            }
                        }
                    }
                }
            }
            else if *op == Token::OP(vec!['-']) {
                match eval_expression(left) {
                    Value::Num(l) => {
                        match eval_expression(right) {
                            Value::Num(r) => {
                                Value::Num(l - r)
                            }
                        }
                    }
                }
            }
            else if *op == Token::OP(vec!['*']) {
                match eval_expression(left) {
                    Value::Num(l) => {
                        match eval_expression(right) {
                            Value::Num(r) => {
                                Value::Num(l * r)
                            }
                        }
                    }
                }
            }
            else if *op == Token::OP(vec!['/']) {
                match eval_expression(left) {
                    Value::Num(l) => {
                        match eval_expression(right) {
                            Value::Num(r) => {
                                Value::Num(l / r)
                            }
                        }
                    }
                }
            }
            else if *op == Token::OP(vec!['%']) {
                match eval_expression(left) {
                    Value::Num(l) => {
                        match eval_expression(right) {
                            Value::Num(r) => {
                                Value::Num(l % r)
                            }
                        }
                    }
                }
            }
            else {
                panic!();
            }
        }
        Expression::UnaryOp(_,_) => {
            panic!();
        }
        Expression::Literal(token) => {
            eval_literal(token)
        }
    }
}

pub fn gen_default_precedence_table() -> BTreeMap<Vec<char>, Precedence> {
    let mut map = BTreeMap::new();
    let plus_chars: Vec<char> = "+".to_string().chars().collect();
    let minus_chars: Vec<char> = "-".to_string().chars().collect();
    let mul_chars: Vec<char> = "*".to_string().chars().collect();
    let div_chars: Vec<char> = "/".to_string().chars().collect();
    let mod_chars: Vec<char> = "%".to_string().chars().collect();
    map.insert(plus_chars, Precedence::LeftAssociative(1));
    map.insert(minus_chars, Precedence::LeftAssociative(1));
    map.insert(mul_chars, Precedence::LeftAssociative(2));
    map.insert(div_chars, Precedence::LeftAssociative(2));
    map.insert(mod_chars, Precedence::LeftAssociative(2));
    map
}
