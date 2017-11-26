use equation::equation::MEquation;
use expr::exprs::MExpr;
use equation::eq_pattern::EPattern;

use std::fmt::{Display, Error, Formatter};

const CONSTANT_NAMES: &str = "ABCEDFGHIJKLMNOPQRSTUVWXYZ";
const VAR_NAMES: &str = "abcedfghijklmnopqrstuvwxyz";


/// A simple action that can be applied to both sides of an equation.
/// The arguments represent the id of the value/variable to do the action with.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Action {
    AddC(u32),
    SubC(u32),
    MulC(u32),
    DivC(u32),

    AddV(u32),
    SubV(u32),
    MulV(u32),
    DivV(u32),
}

/// A way to take actions based of patterns
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PatternAction {
    pub pattern: EPattern,
    pub action: Action
}

impl PatternAction {

    /// Applies the action to both sides of an equation
    pub fn apply(self, eq: MEquation) -> Option<MEquation> {
        let (consts, vars) = self.pattern.bind(eq.clone())?;
        match eq {
            MEquation::Equal(lhs, rhs) => {
                match self.action {
                    Action::AddC(n) => match consts.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Sum(vec![x.clone(), lhs]),
                                MExpr::Sum(vec![x.clone(), rhs]),
                                )),
                        None => None
                    },
                    Action::SubC(n) => match consts.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), lhs]),
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), rhs]),
                                )),
                        None => None
                    },
                    Action::MulC(n) => match consts.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Prod(vec![x.clone(), lhs]),
                                MExpr::Prod(vec![x.clone(), rhs]),
                                )),
                        None => None
                    },
                    Action::DivC(n) => match consts.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Div(box lhs, box x.clone()),
                                MExpr::Div(box rhs, box x.clone()),
                                )),
                        None => None
                    },
                    Action::AddV(n) => match vars.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Sum(vec![x.clone(), lhs]),
                                MExpr::Sum(vec![x.clone(), rhs]),
                                )),
                        None => None
                    },
                    Action::SubV(n) => match vars.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), lhs]),
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), rhs]),
                                )),
                        None => None
                    },
                    Action::MulV(n) => match vars.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Prod(vec![x.clone(), lhs]),
                                MExpr::Prod(vec![x.clone(), rhs]),
                                )),
                        None => None
                    },
                    Action::DivV(n) => match vars.get(&n) {
                        Some(x) => Some(MEquation::Equal(
                                MExpr::Div(box lhs, box x.clone()),
                                MExpr::Div(box rhs, box x.clone()),
                                )),
                        None => None
                    },
                }
            }
        }
    }
}

impl Display for PatternAction {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{} > {}", self.pattern, self.action)
    }
}

impl Display for Action {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Action::AddC(x) => 
                match CONSTANT_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "+{}", name),
                    None => write!(fmt, "+‹{}›", x),
                },
            Action::SubC(x) => 
                match CONSTANT_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "-{}", name),
                    None => write!(fmt, "-‹{}›", x),
                },
            Action::MulC(x) => 
                match CONSTANT_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "*{}", name),
                    None => write!(fmt, "*‹{}›", x),
                },
            Action::DivC(x) => 
                match CONSTANT_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "/{}", name),
                    None => write!(fmt, "/‹{}›", x),
                },
            Action::AddV(x) => 
                match VAR_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "+{}", name),
                    None => write!(fmt, "+‹{}›", x),
                },
            Action::SubV(x) => 
                match VAR_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "-{}", name),
                    None => write!(fmt, "-‹{}›", x),
                },
            Action::MulV(x) => 
                match VAR_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "*{}", name),
                    None => write!(fmt, "*‹{}›", x),
                },
            Action::DivV(x) => 
                match VAR_NAMES.chars().nth(x as usize) {
                    Some(name) => write!(fmt, "/{}", name),
                    None => write!(fmt, "/‹{}›", x),
                },
        }
    }
}
