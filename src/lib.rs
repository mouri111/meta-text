pub fn entry() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Token {
    IDENT(Vec<char>),
    NUM(Vec<char>),
    OP(Vec<char>),
    DELIMITER(Vec<char>),
    STRING(Vec<char>)
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Expression {
    BinaryOp(Box<Expression>,Token,Box<Expression>),
    UnaryOp(Token,Box<Expression>),
    Literal(Token)
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum AST {
    Seq(Vec<Box<AST>>),
    String(Token),
    Expression(Box<Expression>),
    Empty
}

impl AST {
    pub fn new() -> AST {
        AST::Empty
    }
}

fn is_single_char_symbol(c: char) -> bool {
    c == '(' || c == ')' ||
    c == '{' || c == '}' ||
    c == '[' || c == ']' ||
    c == ';' ||
    c == ','
}

fn is_multi_char_symbol(c: char) -> bool {
    c == '!' || c == '#' || c == '$' || c == '%' || c == '&' || c == '*' || c == '+' || c == '-' || c == '.' || c == '/' ||
    c == ':' || c == '<' || c == '=' || c == '>' || c == '?' ||
    c == '@' ||
    c == '\\' || c == '^' ||
    c == '|' || c == '~'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '\'' || c == '_'
}

fn lex(ss: &Vec<char>) -> Vec<Token> {
    let mut res = Vec::<Token>::new();
    let mut i = 0;
    while i < ss.len() {
        if ss[i].is_alphabetic() || ss[i] == '_' {
            let mut buf = vec![];
            while i < ss.len() && is_ident_char(ss[i]) {
                buf.push(ss[i]);
                i += 1;
            }
            res.push(Token::IDENT(buf));
            continue;
        }
        else if ss[i].is_numeric() {
            let mut buf = vec![];
            while i < ss.len() && is_ident_char(ss[i]) {
                buf.push(ss[i]);
                i += 1;
            }
            res.push(Token::NUM(buf));
            continue;
        }
        else if is_single_char_symbol(ss[i]) {
            let mut buf = vec![];
            buf.push(ss[i]);
            i += 1;
            res.push(Token::DELIMITER(buf));
            continue;
        }
        else if is_multi_char_symbol(ss[i]) {
            let mut buf = vec![];
            while i < ss.len() && is_multi_char_symbol(ss[i]) {
                buf.push(ss[i]);
                i += 1;
            }
            res.push(Token::OP(buf));
            continue;
        }
        else if ss[i] == '\'' || ss[i] == '\"' || ss[i] == '`' {
            let bc = ss[i];
            let mut count = 0;
            let mut buf = vec![];
            while i < ss.len() {
                if ss[i] == '\\' {
                    i += 1;
                    if ss[i] == 'n' {
                        buf.push('\n');
                        i += 1;
                    }
                    continue;
                }
                if ss[i] == bc {
                    count += 1;
                }
                buf.push(ss[i]);
                i += 1;
                if count == 2 {
                    break;
                }
            }
            res.push(Token::STRING(buf));
            continue;
        }
        else if ss[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        else {
            panic!("unknown char {}", ss[i]);
        }
    }
    res
}

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Precedence {
    LeftAssociative(i32),
    RightAssociative(i32),
    NonAssociative,
}

impl Precedence {
    fn cont(&self, prec: &Precedence) -> bool {
        match self {
            Precedence::LeftAssociative(p1) => {
                match prec {
                    Precedence::LeftAssociative(p2) => {
                        p1 < p2
                    }
                    Precedence::RightAssociative(p2) => {
                        if p1 == p2 {
                            panic!();
                        }
                        p1 < p2
                    }
                    Precedence::NonAssociative => {
                        panic!()
                    }
                }
            }
            Precedence::RightAssociative(p1) => {
                match prec {
                    Precedence::LeftAssociative(p2) => {
                        if p1 == p2 {
                            panic!();
                        }
                        p1 <= p2
                    }
                    Precedence::RightAssociative(p2) => {
                        p1 <= p2
                    }
                    Precedence::NonAssociative => {
                        panic!();
                    }
                }
            }
            Precedence::NonAssociative => {
                false
            }
        }
    }
}

use std::collections::BTreeMap;
use std::slice::Iter;
use std::iter::Peekable;

pub fn parse_expression(iter: &mut Peekable<Iter<Token>>, precedence_table: &BTreeMap<Vec<char>,Precedence>) -> Expression {
    parse_expression_sub(iter, precedence_table, None)
}

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

pub fn parse(ts: Vec<Token>) -> AST {
    let mut xs = vec![];
    let mut iter = ts.iter();
    while let Some(head) = iter.next() {
        match head {
            Token::STRING(_) => {
                xs.push(Box::new(AST::String(head.clone())));
            }
            _ => {
            }
        }
    }
    let res = AST::Seq(xs);
    res
}

#[derive(PartialEq,Eq,Debug)]
pub enum Value {
    Num(i64),
}

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
