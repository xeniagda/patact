
use math_logic::math_types::*;

impl MExpr {
    /// Attemt to reduce an expression by factoring, evaluating expressions, etc.
    pub fn reduce(self) -> MExpr {
        match self {
            MExpr::Sum(mut terms) => {
                // If there are constants, sum them
                let mut total = 0u64;
                let mut other: Vec<MExpr> = vec![];
                for mut term in terms.drain(..) {
                    match term {
                        MExpr::ConstNum(x) => {
                            total += x;
                        }
                        _ => {
                            other.push(term.reduce());
                        }
                    }
                }
                if other.len() == 0 {
                    return MExpr::ConstNum(total)
                }
                let mut res_terms = vec![];
                if total != 0 {
                    res_terms.push(MExpr::ConstNum(total));
                }
                res_terms.append(&mut other);
                MExpr::Sum(res_terms)
            }
            MExpr::Prod(mut terms) => {
                // Same as for the sum, but with a product instead
                let mut prod = 0u64;
                let mut other: Vec<MExpr> = vec![];
                for mut term in terms.drain(..) {
                    match term {
                        MExpr::ConstNum(x) => {
                            prod *= x;
                        }
                        _ => {
                            other.push(term.reduce());
                        }
                    }
                }
                if other.len() == 0 {
                    return MExpr::ConstNum(prod)
                }
                let mut res_terms = vec![];
                if prod != 0 {
                    res_terms.push(MExpr::ConstNum(prod));
                }
                res_terms.append(&mut other);
                MExpr::Sum(res_terms)
            }
            _ => self
        }
    }
}
