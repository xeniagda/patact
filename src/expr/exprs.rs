use std::cmp::Ordering;
use std::boxed::Box;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum MExpr {
    // A math expression
    Sum(Vec<MExpr>),  // A sum of multiple expressions
    Prod(Vec<MExpr>), // A product of multiple expressions
    Div(Box<MExpr>, Box<MExpr>),
    Exp(Box<MExpr>, Box<MExpr>),

    ConstVar(u32), // A constant variable. Represented by a number
    ConstNum(i64), // A constant integer
    ConstFl(f64),  // A constant number. Should be used only for displaying results

    Var(u32), // A variable, represented by an id
}

impl MExpr {
    // ord_num is a helper for PartialOrd
    pub fn ord_num(&self) -> u8 {
        match *self {
            MExpr::Sum(_) => 0,
            MExpr::Prod(_) => 1,
            MExpr::Div(_, _) => 2,
            MExpr::Exp(_, _) => 3,
            MExpr::ConstVar(_) => 4,
            MExpr::ConstNum(_) => 5,
            MExpr::ConstFl(_) => 6,
            MExpr::Var(_) => 7,
        }
    }
}

impl Ord for MExpr {
    fn cmp(&self, other: &MExpr) -> Ordering {
        match self.ord_num().cmp(&other.ord_num()) {
            Ordering::Equal => match (self, other) {
                (&MExpr::ConstVar(x), &MExpr::ConstVar(y)) | (&MExpr::Var(x), &MExpr::Var(y)) => {
                    x.cmp(&y)
                }

                (&MExpr::ConstNum(x), &MExpr::ConstNum(y)) => x.cmp(&y),
                (&MExpr::ConstFl(x), &MExpr::ConstFl(y)) => {
                    x.partial_cmp(&y).unwrap_or(Ordering::Equal)
                }
                (&MExpr::Sum(ref x), &MExpr::Sum(ref y)) |
                (&MExpr::Prod(ref x), &MExpr::Prod(ref y)) => {
                    for (a, b) in x.iter().zip(y.iter()) {
                        match a.cmp(b) {
                            Ordering::Equal => continue,
                            x => return x,
                        }
                    }
                    x.len().cmp(&y.len())
                }
                (&MExpr::Exp(box ref x, box ref y), &MExpr::Exp(box ref x_, box ref y_)) => {
                    (x, y).cmp(&(x_, y_))
                }
                (&_, &_) => Ordering::Equal,
            },
            x => x,
        }
    }
}

impl PartialOrd for MExpr {
    fn partial_cmp(&self, other: &MExpr) -> Option<Ordering> {
        match self.ord_num().cmp(&other.ord_num()) {
            Ordering::Equal => match (self, other) {
                (&MExpr::ConstVar(x), &MExpr::ConstVar(y)) | (&MExpr::Var(x), &MExpr::Var(y)) => {
                    x.partial_cmp(&y)
                }
                (&MExpr::ConstNum(x), &MExpr::ConstNum(y)) => x.partial_cmp(&y),
                (&MExpr::ConstFl(x), &MExpr::ConstFl(y)) => x.partial_cmp(&y),

                (&MExpr::Sum(ref x), &MExpr::Sum(ref y)) |
                (&MExpr::Prod(ref x), &MExpr::Prod(ref y)) => {
                    for (a, b) in x.iter().zip(y.iter()) {
                        match a.partial_cmp(b) {
                            Some(Ordering::Equal) => continue,
                            x => return x,
                        }
                    }
                    x.len().partial_cmp(&y.len())
                }
                (&MExpr::Exp(box ref x, box ref y), &MExpr::Exp(box ref x_, box ref y_)) => {
                    (x, y).partial_cmp(&(x_, y_))
                }
                (&_, &_) => Some(Ordering::Equal),
            },
            x => Some(x),
        }
    }
}

impl Eq for MExpr {}

// impl PartialEq for MExpr {
//     fn eq(&self, other: &MExpr) -> bool {
//         match (self, other) {
//             (&MExpr::ConstVar(x), &MExpr::ConstVar(y)) | (&MExpr::Var(x), &MExpr::Var(y)) => x == y,
//             (&MExpr::ConstNum(x), &MExpr::ConstNum(y)) => x == y,
//             (&MExpr::ConstFl(x), &MExpr::ConstFl(y)) => x == y,
//             (&MExpr::Sum(ref x), &MExpr::Sum(ref y)) => x == y,
//             (&MExpr::Prod(ref x), &MExpr::Prod(ref y)) => x == y,
//             (&MExpr::Exp(box ref x, box ref y), &MExpr::Exp(box ref x_, box ref y_)) => {
//                 x == x_ && y == y_
//             }
//             (&_, &_) => false,
//         }
//     }
// }


