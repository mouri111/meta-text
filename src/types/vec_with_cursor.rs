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
    pub fn next(&self) -> Result<(&'a T, VecWithCursor<'a, T>), VecWithCursor<'a, T>> {
        if self.p < self.xs.len() {
            let x = &self.xs[self.p];
            let xs = VecWithCursor {
                xs: self.xs,
                p: self.p + 1
            };
            Ok((x,xs))
        }
        else {
            let res = VecWithCursor {
                xs: self.xs,
                p: self.p
            };
            Err(res)
        }
    }
    pub fn is_terminal(&self) -> bool {
        self.p >= self.xs.len()
    }
    pub fn take_while<P>(&self, predicate: P) -> (&'a [T], VecWithCursor<'a, T>)
        where P: FnMut(&T) -> bool {
        let l = self.p;
        let mut r = self.p;
        while r < self.xs.len() && predicate(&self.xs[r]) {
            r += 1;
        }
        (&self.xs[l..r], VecWithCursor {
            xs: self.xs,
            p: r
        })
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
    assert!(xs.next().is_err());
    assert!(xs.is_terminal());
}
