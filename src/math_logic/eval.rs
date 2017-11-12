
use math_logic::math_types::*;
use std::ops::Sub;

impl MExpr {
    /// Attemt to reduce an expression by factoring, evaluating expressions, etc.
    pub fn reduce(self, should_factor: bool) -> MExpr {
        match self.clone() {
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
                            other.push(term.reduce(should_factor));
                        }
                    }
                }
                if other.is_empty() {
                    return MExpr::ConstNum(total)
                }
                let mut res_terms = vec![];
                if total != 0 {
                    res_terms.push(MExpr::ConstNum(total));
                }
                if should_factor {
                    // Factor the rest
                    let gcd = other.iter()
                            .fold(
                                other[0].clone(),
                                | acc, term | acc.gcd_div(term).0
                                 );
                    if gcd == MExpr::ConstNum(1) {
                        res_terms.append(&mut other);
                    } else {
                        other = other.iter()
                            .map(|x| x.gcd_div(&gcd).1.reduce(false))
                            .collect();

                        if other.len() == 1 {
                            res_terms.push(MExpr::Prod(vec![ gcd, other[0].clone() ]).reduce(false));
                        } else {
                            res_terms.push(MExpr::Prod(vec![ gcd, MExpr::Sum(other) ]).reduce(false));
                        }
                    }
                }
                else {
                    res_terms.append(&mut other);
                }
                if res_terms.len() == 1 {
                    res_terms[0].clone()
                } else {
                    MExpr::Sum(res_terms)
                }
            }
            MExpr::Prod(terms) => {
                // Same as for the sum, but with a product instead
                let mut prod = 1u64;
                let mut other: Vec<MExpr> = vec![];

                for mut term in terms.iter()
                        .flat_map(|term| // Unfold nested products
                                  match term.clone().reduce(should_factor) {
                                      MExpr::Prod(terms) => terms,
                                      term => vec![term]
                                  }) {
                    match term.reduce(should_factor) {
                        MExpr::ConstNum(x) => {
                            prod *= x;
                        }
                        term => {
                            other.push(term);
                        }
                    }
                }
                if other.is_empty() {
                    return MExpr::ConstNum(prod)
                }
                let mut res_terms = vec![];
                if prod != 1 {
                    res_terms.push(MExpr::ConstNum(prod));
                }
                res_terms.append(&mut other);
                if res_terms.is_empty() {
                    MExpr::ConstNum(1)
                } else if res_terms.len() == 1 {
                    res_terms[0].clone()
                } else {
                    MExpr::Prod(res_terms)
                }
            }
            MExpr::Div(box num, box den) => {
                if should_factor {
                    num.reduce(should_factor).gcd_div(&den.reduce(should_factor)).1
                } else {
                    num.reduce(should_factor).simple_gcd_div(&den.reduce(should_factor)).1
                }
            }
            _ => self
        }
    }

    /// Like gcd_div but no recursion
    fn simple_gcd_div(&self, other: &MExpr) -> (MExpr, MExpr) {
        match (self.clone(), other.clone()) {
            ( MExpr::ConstNum(a), MExpr::ConstNum(b) ) => {
                let gcd = gcd(a, b);
                if b == gcd {
                    (MExpr::ConstNum(gcd), MExpr::ConstNum(a / gcd))
                } else {
                    (MExpr::ConstNum(gcd), MExpr::Div(box MExpr::ConstNum(a / gcd), box MExpr::ConstNum(b / gcd)))
                }
            }
            ( a, b ) => if a == b { (a, MExpr::ConstNum(1)) } else { (MExpr::ConstNum(1), MExpr::Div(box a, box b)) },
        }
    }
    /// Finds the greatest common divisor of two exrpessions and what their ratio would be. Not perfect, won't find things like
    /// `gcd(x^2 - 4, x^2 - x - 6) == x - 2`
    pub fn gcd_div(&self, other: &MExpr) -> (MExpr, MExpr) {
        match (self.clone(), other.clone()) {
            ( MExpr::ConstNum(_), MExpr::ConstNum(_) ) => self.simple_gcd_div(&other),
              ( MExpr::Prod(factors), x )
            | ( x, MExpr::Prod(factors) ) => {
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
                        continue
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
                let (num, den) = 
                        if let &MExpr::Prod(_) = self {
                            (MExpr::Prod(reduced_factors).reduce(false), reduced_div.reduce(false))
                        } else {
                            (reduced_div.reduce(false), MExpr::Prod(reduced_factors).reduce(false))
                        };

                let ratio = MExpr::Div(box num, box den).reduce(false);

                if gcd.len() == 1 {
                    (gcd[0].clone(), ratio)
                }
                else {
                    (MExpr::Prod(gcd).reduce(false), ratio)
                }
            }
            ( a, b ) => {
                if a == b {
                    (a, MExpr::ConstNum(1))
                } else {
                    (
                        MExpr::ConstNum(1),
                        MExpr::Div(box a, box b).reduce(false)
                    )
                }
            }
        }
    }
}

#[allow(unknown_lints)]
#[allow(eq_op)]
fn gcd<A>(a: A, b: A) -> A
    where
        A: Sub<Output=A>,
        A: Copy,
        A: PartialOrd {

            if a == a - a { // Check if a == 0 for all sub types
        b
    }
    else if a > b {
        gcd(b, a)
    }
    else {
        gcd(b - a, a)
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
        MExpr::ConstVar(0).gcd_div(&MExpr::Prod(vec![ MExpr::ConstVar(1), MExpr::ConstNum(5)])),
        (MExpr::ConstNum(1), MExpr::Div(box MExpr::ConstVar(0), box MExpr::Prod(vec![ MExpr::ConstNum(5), MExpr::ConstVar(1)])))
        );

    assert_eq!(
        MExpr::Prod(vec![MExpr::ConstVar(1), MExpr::ConstNum(10)]).gcd_div(&MExpr::ConstVar(1)),
        (MExpr::ConstVar(1), MExpr::ConstNum(10))
        );

    assert_eq!(
        MExpr::Prod(vec![MExpr::ConstVar(1), MExpr::ConstNum(10)]).gcd_div(&MExpr::Prod(vec![MExpr::ConstVar(1), MExpr::ConstNum(3)])),
        (MExpr::ConstVar(1), MExpr::Div(box MExpr::ConstNum(10), box MExpr::ConstNum(3)))
        );


}
