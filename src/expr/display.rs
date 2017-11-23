use expr::exprs::*;
use expr::expr_pattern::*;

use std::fmt::{Display, Error, Formatter};


const CONSTANT_NAMES: &str = "ABCEDFGHIJKLMNOPQRSTUVWXYZ";
const VAR_NAMES: &str = "abcedfghijklmnopqrstuvwxyz";


impl Display for MExpr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            MExpr::ConstVar(x) => 
                match CONSTANT_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "{}", name),
                    None => write!(fmt, "‹{}›", x),
                },
            MExpr::ConstNum(x) => write!(fmt, "{}", x),
            MExpr::ConstFl(x) => write!(fmt, "{}", x),
            MExpr::Var(x) => match VAR_NAMES.chars().nth(x as usize) {
                Some(name) => write!(fmt, "{}", name),
                None => write!(fmt, "«{}»", x),
            },
            MExpr::Sum(ref terms) => {
                let mut first = true;
                for term in terms {
                    if !first {
                        write!(fmt, " + ")?
                    }
                    first = false;
                    write!(fmt, "{}", term)?
                }
                Ok(())
            }
            MExpr::Prod(ref terms) => {
                let mut first = true;
                for term in terms {
                    if !first {
                        write!(fmt, " * ")?
                    }
                    first = false;
                    if term.ord_num() <= self.ord_num() {
                        write!(fmt, "({})", term)?
                    } else {
                        write!(fmt, "{}", term)?
                    }
                }
                Ok(())
            }
            MExpr::Exp(box ref base, box ref exp) => {
                if base.ord_num() <= self.ord_num() {
                    write!(fmt, "({})", base)?
                } else {
                    write!(fmt, "{}", base)?
                }

                write!(fmt, " ^ ")?;

                if exp.ord_num() <= self.ord_num() {
                    write!(fmt, "({})", exp)?
                } else {
                    write!(fmt, "{}", exp)?
                }
                Ok(())
            }
            MExpr::Div(box ref base, box ref exp) => {
                if base.ord_num() <= self.ord_num() {
                    write!(fmt, "({})", base)?
                } else {
                    write!(fmt, "{}", base)?
                }

                write!(fmt, " / ")?;

                if exp.ord_num() <= self.ord_num() {
                    write!(fmt, "({})", exp)?
                } else {
                    write!(fmt, "{}", exp)?
                }
                Ok(())
            }
        }
    }
}

impl Display for MPattern {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self.clone() {
            MPattern::Const(id) => {
                match CONSTANT_NAMES.chars().nth(id as usize) {
                    Some(name) => write!(fmt, "{}", name),
                    None => write!(fmt, "‹{}›", id),
                }
            }
            MPattern::Var(id) => {
                match VAR_NAMES.chars().nth(id as usize) {
                    Some(name) => write!(fmt, "{}", name),
                    None => write!(fmt, "‹{}›", id),
                }
            }
            MPattern::Sum(terms) => {
                let mut first = true;
                for term in terms {
                    if !first {
                        write!(fmt, " + ")?;
                    }
                    first = false;
                    write!(fmt, "{}", term)?;
                }
                Ok(())
            }
            MPattern::Prod(factors) => {
                let mut first = true;
                for factor in factors {
                    if !first {
                        write!(fmt, " * ")?;
                    }
                    first = false;
                    write!(fmt, "({})", factor)?;
                }
                Ok(())
            }
            MPattern::Div(box num, box den) => {
                write!(fmt, "({}) / ({})", num, den)
            }
        }
    }
}