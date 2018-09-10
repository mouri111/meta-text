#[derive(Debug,PartialEq,Eq,Clone)]
pub struct VecWithCursor<'a, T: 'a> {
    xs: &'a Vec<T>,
    p: usize,
}

impl<'a, T> VecWithCursor<'a, T> {
    pub fn new(xs: &'a Vec<T>) -> VecWithCursor<'a, T> {
        VecWithCursor {
            xs,
            p: 0
        }
    }
}
