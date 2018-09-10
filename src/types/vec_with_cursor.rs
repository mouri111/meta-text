#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VecWithCursor<'a, T: 'a> {
    xs: &'a Vec<T>,
    p: usize,
}
