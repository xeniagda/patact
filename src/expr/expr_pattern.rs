use std::collections::HashMap;
use expr::exprs::MExpr;

use utils::merge;

/// A pattern that matches variables and constants in expressions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MPattern {
    Const(u32),
    Var(u32),
    Sum(Vec<MPattern>),
    Prod(Vec<MPattern>),
    Div(Box<MPattern>, Box<MPattern>),
}

impl MExpr {
    fn is_const(&self) -> bool {
        match self {
            &MExpr::Var(_) => false,
            &MExpr::Sum(ref terms) | &MExpr::Prod(ref terms) => {
                terms.iter().all(|x| x.is_const())
            }
            &MExpr::Div(box ref a, box ref b) | &MExpr::Exp(box ref a, box ref b) => {
                a.is_const() && b.is_const()
            }
            _ => true
        }
    }
}

impl MPattern {

    pub fn trivial_reduce(self) -> MPattern {
        match self {
            MPattern::Sum(terms) => {
                // Reduce each term
                let terms = terms.into_iter().map(|term| term.trivial_reduce()).collect::<Vec<_>>();
                
                // Check for sums in terms
                let mut res_terms = vec![];
                for term in terms {
                    match term {
                        MPattern::Sum(subterms) =>
                            res_terms.extend(subterms),
                        x => res_terms.push(x)
                    }
                }

                if res_terms.len() == 1 {
                    res_terms[0].clone()
                } else {
                    MPattern::Sum(res_terms)
                }
            }
            MPattern::Prod(factors) => {
                // Reduce each factor
                let factors = factors.into_iter().map(|factor| factor.trivial_reduce()).collect::<Vec<_>>();
                
                // Check for prods in factors
                let mut res_factors = vec![];
                for factor in factors {
                    match factor {
                        MPattern::Prod(subfactors) =>
                            res_factors.extend(subfactors),
                        x => res_factors.push(x)
                    }
                }

                if res_factors.len() == 1 {
                    res_factors[0].clone()
                } else {
                    MPattern::Prod(res_factors)
                }
            }
            MPattern::Div(box num, box den) => {
                MPattern::Div(box num.trivial_reduce(), box den.trivial_reduce())
            }
            x => x
        }
    }

    /// Tries to match a pattern to an expression, binding all variables in the pattern.
    /// Pseudo-example: `(A: const + b: var).bind(2x + 1) -> {A: const -> 1, b: var -> 2x}`
    pub fn bind(self, expr: MExpr) -> Option<(HashMap<u32, MExpr>, HashMap<u32, MExpr>)> {
        let mut const_res: HashMap<u32, MExpr> = HashMap::new();
        let mut var_res:   HashMap<u32, MExpr> = HashMap::new();
        match (self.trivial_reduce(), expr.trivial_reduce()) {
            (MPattern::Const(n), other) => {
                if other.is_const() {
                    const_res.insert(n, other);
                } else {
                    return None;
                }
            }
            (MPattern::Var(n), other) => {
                if other.is_const() {
                    return None;
                } else {
                    var_res.insert(n, other);
                }
            }
            (MPattern::Div(box a_pat, box b_pat), MExpr::Div(box a, box b)) => {
                let (const_res_a, var_res_a) = a_pat.bind(a)?;
                let (const_res_b, var_res_b) = b_pat.bind(b)?;
                merge(&mut const_res, const_res_a)?;
                merge(&mut const_res, const_res_b)?;
                merge(&mut var_res, var_res_a)?;
                merge(&mut var_res, var_res_b)?;
            }
            (MPattern::Sum(pterms), other) => {
                // println!("pterms = {:?}, other = {:?}", pterms, other);
                match other {
                    MExpr::Sum(terms) => {
                        let mut worked = false;
                        for (i_p, pattern) in pterms.clone().into_iter().enumerate() {
                            if worked { break }
                            for (i, elem) in terms.clone().into_iter().enumerate() {
                                // println!("i = {}, pattern = {:?}, elem = {:?}", i, pterms[0], elem);
                                if let Some((const_res_f, var_res_f)) = pattern.clone().bind(elem.clone()) {
                                    // println!("Matched, const = {:?}, var = {:?}", const_res_f, var_res_f);

                                    let mut other_terms: Vec<MExpr> = terms[0..i].to_vec();
                                    for a in &terms[i+1..] {
                                        other_terms.push(a.clone());
                                    }

                                    let mut other_pterms: Vec<MPattern> = pterms[0..i_p].to_vec();
                                    for a in &pterms[i_p+1..] {
                                        other_pterms.push(a.clone());
                                    }

                                    // println!("rest pattern = {:?}, expr = {:?}", MPattern::Sum(other_pterms.clone()), MExpr::Sum(other_terms.clone()));
                                    if let Some((const_res_r, var_res_r)) =
                                        MPattern::Sum(other_pterms).bind(MExpr::Sum(other_terms)) {
                                            merge(&mut const_res, const_res_f)?;
                                            merge(&mut const_res, const_res_r)?;
                                            merge(&mut var_res, var_res_f)?;
                                            merge(&mut var_res, var_res_r)?;
                                            worked = true;
                                            // println!("Worked!");
                                            break;
                                        }
                                }
                            }
                        }
                        // println!("worked = {}", worked);
                        if !worked {
                            return None;
                        }
                    }
                    _ => return None
                }
            }
            (MPattern::Prod(pterms), other) => {
                // println!("pterms = {:?}, other = {:?}", pterms, other);
                match other {
                    MExpr::Prod(terms) => {
                        let mut worked = false;
                        for (i_p, pattern) in pterms.clone().into_iter().enumerate() {
                            if worked { break }
                            for (i, elem) in terms.clone().into_iter().enumerate() {
                                // println!("i = {}, pattern = {:?}, elem = {:?}", i, pterms[0], elem);
                                if let Some((const_res_f, var_res_f)) = pattern.clone().bind(elem.clone()) {
                                    // println!("Matched, const = {:?}, var = {:?}", const_res_f, var_res_f);

                                    let mut other_terms: Vec<MExpr> = terms[0..i].to_vec();
                                    for a in &terms[i+1..] {
                                        other_terms.push(a.clone());
                                    }

                                    let mut other_pterms: Vec<MPattern> = pterms[0..i_p].to_vec();
                                    for a in &pterms[i_p+1..] {
                                        other_pterms.push(a.clone());
                                    }

                                    if let Some((const_res_r, var_res_r)) =
                                        MPattern::Prod(other_pterms).bind(MExpr::Prod(other_terms)) {
                                            merge(&mut const_res, const_res_f)?;
                                            merge(&mut const_res, const_res_r)?;
                                            merge(&mut var_res, var_res_f)?;
                                            merge(&mut var_res, var_res_r)?;
                                            worked = true;
                                            break;
                                        }
                                }
                            }
                        }
                        if !worked {
                            return None;
                        }
                    }
                    _ => return None
                }
            }
            _ => return None
        };
        Some((const_res, var_res))
    }
    /// Used by `is_subpattern`
    fn convert_to_mexpr(self) -> MExpr {
        match self {
            MPattern::Const(x) => MExpr::ConstVar(x),
            MPattern::Var(x) => MExpr::Var(x),
            MPattern::Sum(terms) => {
                let converted_terms = terms.into_iter()
                        .map(|term| term.convert_to_mexpr())
                        .collect();
                MExpr::Sum(converted_terms)
            }
            MPattern::Prod(factors) => {
                let converted_factors = factors.into_iter()
                        .map(|factor| factor.convert_to_mexpr())
                        .collect();
                MExpr::Sum(converted_factors)
            }
            MPattern::Div(box den, box num) => {
                MExpr::Div(
                    box den.convert_to_mexpr(),
                    box num.convert_to_mexpr()
                    )
            }
        }
    }
    /// Checks if this pattern is a "sub-pattern" of the `other`.
    /// A pattern is a sub-pattern of this if all the expressions matched by this pattern will be
    /// matched by that pattern too.
    pub fn is_subpattern_of(self, other: MPattern) -> bool {
        other.bind(self.convert_to_mexpr()).is_some()
    }
}

impl MExpr {
    pub fn generate_patterns_with_idx(&self, var_idx: u32) -> (Vec<MPattern>, u32) {
        match self.clone() {
            MExpr::ConstVar(_) 
            | MExpr::ConstNum(_) 
            | MExpr::ConstFl(_) => ( vec![MPattern::Const(var_idx) ], var_idx + 1 ),
            MExpr::Var(_) => ( vec![MPattern::Var(var_idx)], var_idx + 1 ),
            MExpr::Div(box num, box den) => {
                let (num_pats, var_idx) = num.generate_patterns_with_idx(var_idx);
                let (den_pats, var_idx) = den.generate_patterns_with_idx(var_idx);
                let mut res = vec![];
                for num_pat in num_pats {
                    for den_pat in den_pats.clone() {
                        res.push(MPattern::Div(box num_pat.clone(), box den_pat));
                    }
                }
                if !self.is_const() {
                    res.push(MPattern::Var(var_idx));
                    ( res, var_idx + 1 )
                } else {
                    ( res, var_idx )
                }
            }
            MExpr::Exp(box _, box _) => ( vec![], var_idx ),
            MExpr::Sum(terms) => {
                let (first, rest) = (terms[0].clone(), &terms[1..]);
                let (first_pats, var_idx) = first.generate_patterns_with_idx(var_idx);
                let (rest_pats, var_idx) = MExpr::Sum(rest.to_vec()).reduce(false).generate_patterns_with_idx(var_idx);

                let mut res = vec![];
                for first_pat in first_pats {
                    for rest_pat in rest_pats.clone() {
                        res.push(MPattern::Sum(vec![first_pat.clone(), rest_pat]).trivial_reduce());
                    }
                }
                if !self.is_const() {
                    res.push(MPattern::Var(var_idx));
                    ( res, var_idx + 1 )
                } else {
                    ( res, var_idx )
                }
            }
            MExpr::Prod(terms) => {
                let (first, rest) = (terms[0].clone(), &terms[1..]);
                let (first_pats, var_idx) = first.generate_patterns_with_idx(var_idx);
                let (rest_pats, var_idx) = MExpr::Prod(rest.to_vec()).reduce(false).generate_patterns_with_idx(var_idx);

                let mut res = vec![];
                for first_pat in first_pats {
                    for rest_pat in rest_pats.clone() {
                        res.push(MPattern::Prod(vec![first_pat.clone(), rest_pat]).trivial_reduce());
                    }
                }
                if !self.is_const() {
                    res.push(MPattern::Var(var_idx));
                    ( res, var_idx + 1 )
                } else {
                    ( res, var_idx )
                }
            }
        }
    }
}

#[test]
fn test_bind() {
    let pattern = MPattern::Prod(vec![MPattern::Const(0), MPattern::Var(0)]);
    let expr = MExpr::Prod(vec![MExpr::Var(0), MExpr::ConstNum(2)]);
    let bind = pattern.bind(expr);
    assert!(bind.is_some());
    let bind = bind.unwrap();
    assert_eq!(bind.0.get(&0), Some(&MExpr::ConstNum(2)));
    assert_eq!(bind.1.get(&0), Some(&MExpr::Var(0)));

    let pattern = MPattern::Div(box MPattern::Const(0), box MPattern::Var(0));
    let expr = MExpr::Div(box MExpr::ConstNum(3), box MExpr::Prod(vec![ MExpr::ConstNum(2), MExpr::Var(21)] ));
    let bind = pattern.bind(expr);
    assert!(bind.is_some());
    let bind = bind.unwrap();
    assert_eq!(bind.0.get(&0), Some(&MExpr::ConstNum(3)));
    assert_eq!(bind.1.get(&0), Some(&MExpr::Prod(vec![ MExpr::ConstNum(2), MExpr::Var(21)] )));

    let pattern = MPattern::Sum(vec![MPattern::Const(0), MPattern::Var(0)]);
    let expr = MExpr::Sum(vec![MExpr::Var(0), MExpr::ConstNum(2)]);
    let bind = pattern.bind(expr);
    assert!(bind.is_some());
    let bind = bind.unwrap();
    assert_eq!(bind.0.get(&0), Some(&MExpr::ConstNum(2)));
    assert_eq!(bind.1.get(&0), Some(&MExpr::Var(0)));
}

#[test]
fn test_subpatterns() {
    let p1 = "a + b".parse::<MPattern>().unwrap().trivial_reduce();
    let p2 = "a".parse::<MPattern>().unwrap().trivial_reduce();
    assert!(p1.clone().is_subpattern_of(p2.clone()));
    assert!(!p2.clone().is_subpattern_of(p1.clone()));


    let p1 = "a / X + A * b".parse::<MPattern>().unwrap().trivial_reduce();
    let p2 = "a + b".parse::<MPattern>().unwrap().trivial_reduce();
    assert!(p1.clone().is_subpattern_of(p2.clone()));
    assert!(!p2.clone().is_subpattern_of(p1.clone()));


    let p1 = "(a + X + b) / a".parse::<MPattern>().unwrap().trivial_reduce();
    let p2 = "(a + b) / a".parse::<MPattern>().unwrap().trivial_reduce();
    assert!(p1.clone().is_subpattern_of(p2.clone()));
    assert!(!p2.clone().is_subpattern_of(p1.clone()));

    let p1 = "(a + X + b) / (a + B)".parse::<MPattern>().unwrap().trivial_reduce();
    let p2 = "(a + b) / a".parse::<MPattern>().unwrap().trivial_reduce();
    assert!(!p1.clone().is_subpattern_of(p2.clone()));
    assert!(!p2.clone().is_subpattern_of(p1.clone()));
}
