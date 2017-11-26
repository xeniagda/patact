
use equation::equation::MEquation;
use equation::action::{PatternAction, Action};
use expr::expr_pattern::MPattern;
use expr::exprs::MExpr;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use utils::merge;

// A pattern for equations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EPattern {
    PEq(MPattern, MPattern)
}

impl Display for EPattern {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            &EPattern::PEq(ref lhs, ref rhs) => {
                write!(fmt, "{} = {}", lhs, rhs)
            }
        }
    }
}

impl EPattern {
    /// Reduces both sides of the pattern
    pub fn trivial_reduce(self) -> EPattern {
        match self {
            EPattern::PEq(lhs, rhs) => {
                EPattern::PEq(lhs.trivial_reduce(), rhs.trivial_reduce())
            }
        }
    }

    /// Binds both sides of the pattern
    pub fn bind(self, other: MEquation) -> Option<(HashMap<u32, MExpr>, HashMap<u32, MExpr>)> {
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
        Some((const_res, var_res))
    }

    /// Checks if this pattern is a "sub-pattern" of the `other`.
    /// A pattern is a sub-pattern of this if all the expressions matched by this pattern will be
    /// matched by that pattern too.
    pub fn is_subpattern_of(self, other: EPattern) -> bool {
        match (self, other) {
            (EPattern::PEq(lhs1, rhs1), EPattern::PEq(lhs2, rhs2)) => {
                lhs1.is_subpattern_of(lhs2) && rhs1.is_subpattern_of(rhs2)
            }
        }
    }

    /// Gets all the variables and constants in the pattern
    pub fn get_free(self) -> (Vec<u32>, Vec<u32>) {
        match self {
            EPattern::PEq(lhs, rhs) => {
                let (mut lconsts, mut lvars) = lhs.get_free();
                let (mut rconsts, mut rvars) = rhs.get_free();
                lconsts.append(&mut rconsts);
                lvars.append(&mut rvars);
                (lconsts, lvars)
            }
        }
    }
}

impl MEquation {

    /// Generates patacts from this equation
    pub fn generate_patacts(self) -> Vec<PatternAction> {
        let pats = self.generate_patterns();
        let mut patacts = vec![];
        for pat in pats {
            let (consts, vars) = pat.clone().get_free();
            for var in vars {
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::AddV(var) } );
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::SubV(var) } );
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::MulV(var) } );
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::DivV(var) } );
            }
            for constant in consts {
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::AddC(constant) } );
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::SubC(constant) } );
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::MulC(constant) } );
                patacts.push( PatternAction { pattern: pat.clone(), action: Action::DivC(constant) } );
            }
        }
        patacts
    }

    pub fn generate_patterns(self) -> Vec<EPattern> {
        match self {
            MEquation::Equal(lhs, rhs) => {
                let (lhs_pats, var_idx) = lhs.generate_patterns_with_idx(0);
                let rhs_pats = rhs.generate_patterns_with_idx(var_idx).0;
                let mut res = vec![];
                for lhs_pat in lhs_pats {
                    for rhs_pat in rhs_pats.clone() {
                        res.push(EPattern::PEq(lhs_pat.clone(), rhs_pat));
                    }
                }
                res

            }
        }
    }
}

#[test]
fn test_subpatterns() {
    let p1 = "a + b = c + d".parse::<EPattern>().unwrap();
    let p2 = "a = b".parse::<EPattern>().unwrap();
    assert!(p1.clone().is_subpattern_of(p2.clone()));
    assert!(!p2.clone().is_subpattern_of(p1.clone()));


    let p1 = "a / X + A * b = a / X".parse::<EPattern>().unwrap();
    let p2 = "a + b = a".parse::<EPattern>().unwrap();
    assert!(p1.clone().is_subpattern_of(p2.clone()));
    assert!(!p2.clone().is_subpattern_of(p1.clone()));
}
