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
pub fn render(ss: String) -> String {
    unimplemented!();
}
