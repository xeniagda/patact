
use expr::exprs::MExpr;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone)]
pub enum MEquation {
    Equal(MExpr, MExpr)
}

impl MEquation {
    pub fn reduce(self) -> MEquation {
        match self {
            MEquation::Equal(lhs, rhs) => {
                MEquation::Equal(lhs.reduce(true), rhs.reduce(true))
            }
        }
    }
    pub fn trivial_reduce(self) -> MEquation {
        match self {
            MEquation::Equal(lhs, rhs) => {
                MEquation::Equal(lhs.trivial_reduce(), rhs.trivial_reduce())
            }
        }
    }
}

impl Eq for MEquation {}

impl PartialEq for MEquation {
    fn eq(&self, other: &MEquation) -> bool {
        match (self, other) {
            (&MEquation::Equal(ref lhs1, ref rhs1), &MEquation::Equal(ref lhs2, ref rhs2)) => {
                lhs1 == lhs2 && rhs1 == rhs2
            }
        }
    }
}

impl Display for MEquation {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            &MEquation::Equal(ref lhs, ref rhs) => {
                write!(fmt, "{} = {}", lhs, rhs)
            }
        }
    }
}   

#[test]
fn test() {
    assert_eq!(MEquation::Equal(MExpr::ConstNum(1), MExpr::ConstNum(2)), MEquation::Equal(MExpr::ConstNum(1), MExpr::ConstNum(2)));
}
