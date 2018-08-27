pub mod atom;
pub mod expression;

use types::token::*;
use types::ast::*;

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
