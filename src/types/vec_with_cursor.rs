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
    pub fn next(&self) -> Option<(&'a T, VecWithCursor<'a, T>)> {
        unimplemented!();
    }
}

#[test]
fn test_next() {
    let xs = vec![1, 2, 3];
    let xs = VecWithCursor::new(&xs);
    let (x, xs) = xs.next().unwrap();
    assert_eq!(1, *x);
    let (x, xs) = xs.next().unwrap();
    assert_eq!(2, *x);
    let (x, xs) = xs.next().unwrap();
    assert_eq!(3, *x);
    assert!(xs.next().is_none());
}
