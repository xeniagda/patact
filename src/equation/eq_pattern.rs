
use equation::equation::MEquation;
use expr::expr_pattern::MPattern;
use expr::exprs::MExpr;
use std::collections::HashMap;

use utils::merge;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EPattern {
    PEq(MPattern, MPattern)
}

impl EPattern {
    pub fn trivial_reduce(self) -> EPattern {
        match self {
            EPattern::PEq(lhs, rhs) => {
                EPattern::PEq(lhs.trivial_reduce(), rhs.trivial_reduce())
            }
        }
    }

    pub fn bind(self, other: MEquation) -> Result<(HashMap<u32, MExpr>, HashMap<u32, MExpr>), ()> {
        let mut const_res: HashMap<u32, MExpr> = HashMap::new();
        let mut var_res:   HashMap<u32, MExpr> = HashMap::new();
        match (self.trivial_reduce(), other.trivial_reduce()) {
            (EPattern::PEq(p_lhs, p_rhs), MEquation::Equal(lhs, rhs)) => {
                let (c_l, v_l) = p_lhs.bind(lhs)?;
                let (c_r, v_r) = p_rhs.bind(rhs)?;
                merge(&mut const_res, c_l)?;
                merge(&mut const_res, c_r)?;
                merge(&mut var_res, v_l)?;
                merge(&mut var_res, v_r)?;
            }
        };
        Ok((const_res, var_res))
    }
}
