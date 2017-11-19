
use expr::exprs::*;
use expr::reduce_prod;
use expr::reduce_sum;


impl MExpr {

    /// A very light reduction that only unfolds nested expressions and changes empty sums and
    /// products
    pub fn trivial_reduce(self) -> MExpr {
        match self.clone() {
            MExpr::Sum(terms) => {
                let terms = reduce_sum::unfold_nested(terms);

                // Reduce every sub-expression
                let terms: Vec<_> = terms.into_iter().map(|term| term.trivial_reduce()).collect();

                if terms.is_empty() {
                    MExpr::ConstNum(0)
                } else if terms.len() == 1 {
                    terms[0].clone()
                } else {
                    MExpr::Sum(terms)
                }
            }       
            MExpr::Prod(terms) => {
                let terms = reduce_prod::unfold_nested(terms);

                // Reduce every sub-expression
                let terms: Vec<_> = terms.into_iter().map(|term| term.trivial_reduce()).collect();

                if terms.is_empty() {
                    MExpr::ConstNum(1)
                } else if terms.len() == 1 {
                    terms[0].clone()
                } else {
                    MExpr::Prod(terms)
                }
            }
            MExpr::Div(box num, box den) => {
                MExpr::Div(box num.trivial_reduce(), box den.trivial_reduce())
            }
            _ => self
        }
    }

    /// Attemt to reduce an expression by factoring, evaluating expressions, etc.
    pub fn reduce(self, should_factor: bool) -> MExpr {
        match self.clone() {
            MExpr::Sum(terms) => {
                let terms = reduce_sum::unfold_nested(terms);

                // Reduce every sub-expression
                let terms = terms.into_iter().map(|term| term.reduce(should_factor)).collect();

                let mut terms = reduce_sum::combine_constants(terms);

                let mut res_terms = vec![];
                if should_factor && terms.len() > 1 {
                    // Factor the rest
                    let gcd = terms
                        .iter()
                        .fold(terms[0].clone(), |acc, term| acc.gcd_div(term).0);
                    if gcd == MExpr::ConstNum(1) {
                        res_terms.append(&mut terms);
                    } else {
                        let terms: Vec<_> = terms
                            .iter()
                            .map(|x| x.gcd_div(&gcd).1.reduce(false))
                            .collect();

                        if terms.len() == 1 {
                            res_terms.push(MExpr::Prod(vec![gcd, terms[0].clone()]).reduce(false));
                        } else {
                            res_terms.push(MExpr::Prod(vec![gcd, MExpr::Sum(terms)]).reduce(false));
                        }
                    }
                } else {
                    res_terms.append(&mut terms);
                }
                if res_terms.is_empty() {
                    MExpr::ConstNum(0)
                } else if res_terms.len() == 1 {
                    res_terms[0].clone()
                } else {
                    MExpr::Sum(res_terms)
                }
            }
            MExpr::Prod(terms) => {
                let terms = reduce_prod::unfold_nested(terms);

                // Reduce every sub-expression
                let terms = terms.into_iter().map(|term| term.reduce(should_factor)).collect();


                let terms = reduce_prod::combine_constants(terms);

                if let Some( (num, den) ) = reduce_prod::unfold_division(terms.clone()) {
                    MExpr::Div(box MExpr::Prod(num), box MExpr::Prod(den)).reduce(should_factor)
                } else {
                    if terms.is_empty() {
                        MExpr::ConstNum(1)
                    } else if terms.len() == 1 {
                        terms[0].clone()
                    } else {
                        MExpr::Prod(terms)
                    }
                }
            }
            MExpr::Div(box num, box den) => {
                let gcd_div =
                    if should_factor {
                        num.reduce(should_factor)
                            .gcd_div(&den.reduce(should_factor))
                            .1

                    } else {
                        num.reduce(should_factor)
                            .simple_gcd_div(&den.reduce(should_factor))
                            .1
                    };
                match gcd_div {
                    MExpr::Div(box num, box den) => {
                        if den == MExpr::ConstNum(1) {
                            num
                        }
                        else {
                            MExpr::Div(box num, box den)
                        }
                    }
                    other => other
                }
            }
            _ => self,
        }
    }

    /// Like gcd_div but no recursion
    fn simple_gcd_div(&self, other: &MExpr) -> (MExpr, MExpr) {
        match (self.clone(), other.clone()) {
            (MExpr::ConstNum(a), MExpr::ConstNum(b)) => {
                let gcd = gcd(a, b);
                if b == gcd {
                    (MExpr::ConstNum(gcd), MExpr::ConstNum(a / gcd))
                } else {
                    (
                        MExpr::ConstNum(gcd),
                        MExpr::Div(box MExpr::ConstNum(a / gcd), box MExpr::ConstNum(b / gcd)),
                    )
                }
            }
            (a, b) => if a == b {
                (a, MExpr::ConstNum(1))
            } else {
                (MExpr::ConstNum(1), MExpr::Div(box a, box b))
            },
        }
    }
    /// Finds the greatest common divisor of two exrpessions and what their ratio would be. Not
    /// perfect, won't find things like `gcd(x^2 - 4, x^2 - x - 6) == x - 2`
    pub fn gcd_div(&self, other: &MExpr) -> (MExpr, MExpr) {
        match (self.clone(), other.clone()) {
            (MExpr::ConstNum(_), MExpr::ConstNum(_)) => self.simple_gcd_div(&other),
            (MExpr::Prod(factors), x) | (x, MExpr::Prod(factors)) => {
                let mut gcd = vec![];
                let mut reduced_factors = vec![];
                let mut reduced_div = x.clone();
                for factor in factors {
                    let (g, _) = factor.gcd_div(&x);

                    let (_, ratio) = factor.gcd_div(&reduced_div);
                    if let MExpr::Div(box a, box b) = ratio {
                        reduced_factors.push(a);
                        reduced_div = b;
                    } else {
                        reduced_factors.push(ratio);
                        reduced_div = MExpr::ConstNum(1);
                    }
                    if g == MExpr::ConstNum(1) {
                        continue;
                    }
                    match g {
                        MExpr::Prod(mut x) => {
                            gcd.append(&mut x);
                        }
                        _ => {
                            gcd.push(g);
                        }
                    }
                }
                let (num, den) = if let &MExpr::Prod(_) = self {
                    (
                        MExpr::Prod(reduced_factors).reduce(false),
                        reduced_div.reduce(false),
                    )
                } else {
                    (
                        reduced_div.reduce(false),
                        MExpr::Prod(reduced_factors).reduce(false),
                    )
                };

                let ratio = MExpr::Div(box num, box den).reduce(false);

                if gcd.len() == 1 {
                    (gcd[0].clone(), ratio)
                } else {
                    (MExpr::Prod(gcd).reduce(false), ratio)
                }
            }
            (a, b) =>
                if a == b {
                    (a, MExpr::ConstNum(1))
                } else {
                    (MExpr::ConstNum(1), MExpr::Div(box a, box b).reduce(false))
                },
        }
    }
}

/// Simple gcd algorithm
fn gcd(a: i64, b: i64) -> i64 {
    if a < 0 && b < 0 {
        -gcd(-a, -b)
    } else if a < 0 {
        gcd(-a, b)
    } else if b < 0 {
        gcd(a, -b)
    }
    else {
        if a == 0 {
            // Check if a == 0 for all sub types
            b
        } else if a > b {
            gcd(b, a)
        } else {
            gcd(b - a, a)
        }
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(10, 5), 5);
    assert_eq!(gcd(232, 100), 4);
    assert_eq!(gcd(420, 69), 3);
    assert_eq!(gcd(420, 71), 1);

    assert_eq!(
        MExpr::Div(box MExpr::ConstNum(10), box MExpr::ConstNum(5)).reduce(true),
        MExpr::ConstNum(2)
        );

    assert_eq!(
        MExpr::Div(box MExpr::Var(0), box MExpr::ConstNum(1)).reduce(true),
        MExpr::Var(0)
        );

    assert_eq!(
        MExpr::Prod(vec![MExpr::ConstVar(1), MExpr::ConstNum(10)]).gcd_div(&MExpr::ConstVar(1)),
        (MExpr::ConstVar(1), MExpr::ConstNum(10))
        );

    assert_eq!(
        MExpr::Sum(vec![MExpr::ConstNum(-5), MExpr::ConstNum(5)]).reduce(true),
        MExpr::ConstNum(0)
        );

    assert_eq!(
        MExpr::Prod(vec! [MExpr::Var(0), MExpr::Div(box MExpr::ConstNum(3), box MExpr::Var(0))]).reduce(true),
        MExpr::ConstNum(3)
        );
}
