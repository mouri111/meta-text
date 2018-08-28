use types::token::*;

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

pub fn lex(ss: &Vec<char>) -> Vec<Token> {
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
