
use equation::equation::MEquation;
use expr::exprs::MExpr;
use equation::eq_pattern::EPattern;

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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PatternAction {
    pub pattern: EPattern,
    pub action: Action
}

impl PatternAction {
    pub fn apply(self, eq: MEquation) -> Result<MEquation, ()> {
        let (consts, vars) = self.pattern.bind(eq.clone())?;
        match eq {
            MEquation::Equal(lhs, rhs) => {
                match self.action {
                    Action::AddC(n) => match consts.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Sum(vec![x.clone(), lhs]),
                                MExpr::Sum(vec![x.clone(), rhs]),
                                )),
                        None => Err(())
                    },
                    Action::SubC(n) => match consts.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), lhs]),
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), rhs]),
                                )),
                        None => Err(())
                    },
                    Action::MulC(n) => match consts.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Prod(vec![x.clone(), lhs]),
                                MExpr::Prod(vec![x.clone(), rhs]),
                                )),
                        None => Err(())
                    },
                    Action::DivC(n) => match consts.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Div(box x.clone(), box lhs),
                                MExpr::Div(box x.clone(), box rhs),
                                )),
                        None => Err(())
                    },
                    Action::AddV(n) => match vars.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Sum(vec![x.clone(), lhs]),
                                MExpr::Sum(vec![x.clone(), rhs]),
                                )),
                        None => Err(())
                    },
                    Action::SubV(n) => match vars.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), lhs]),
                                MExpr::Sum(vec![MExpr::Prod(vec![MExpr::ConstNum(-1), x.clone()]), rhs]),
                                )),
                        None => Err(())
                    },
                    Action::MulV(n) => match vars.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Prod(vec![x.clone(), lhs]),
                                MExpr::Prod(vec![x.clone(), rhs]),
                                )),
                        None => Err(())
                    },
                    Action::DivV(n) => match vars.get(&n) {
                        Some(x) => Ok(MEquation::Equal(
                                MExpr::Div(box x.clone(), box lhs),
                                MExpr::Div(box x.clone(), box rhs),
                                )),
                        None => Err(())
                    },
                }
            }
        }
    }
}