pub fn entry() {
    println!("Hello, world!");
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Token {
    IDENT(Vec<char>),
    NUM(Vec<char>),
    OP(Vec<char>),
    DELIMITER(Vec<char>),
    STRING(Vec<char>),
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum AST {
    Seq(Vec<Box<AST>>),
    String(Token),
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

fn lex(ss: String) -> Vec<Token> {
    let ss: Vec<char> = ss.chars().collect();
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
pub fn render(ss: String) -> String {
    unimplemented!();
}
