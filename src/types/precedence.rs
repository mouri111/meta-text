#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub enum Precedence {
    LeftAssociative(i32),
    RightAssociative(i32),
    NonAssociative,
}

impl Precedence {
    pub fn cont(&self, prec: &Precedence) -> bool {
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
