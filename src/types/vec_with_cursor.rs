#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VecWithCursor<T> {
    xs: Vec<T>,
    p: usize,
}
