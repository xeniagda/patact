use std::collections::HashMap;
use expr::exprs::MExpr;

use utils::merge;

/// A pattern that matches variables and constants in expressions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MPattern {
    PConst(u32),
    PVar(u32),
    PSum(Vec<MPattern>),
    PProd(Vec<MPattern>),
    PDiv(Box<MPattern>, Box<MPattern>),
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
            MPattern::PSum(terms) => {
                if terms.len() == 1 {
                    terms[0].clone()
                } else {
                    MPattern::PSum(terms)
                }
            }
            MPattern::PProd(terms) => {
                if terms.len() == 1 {
                    terms[0].clone()
                } else {
                    MPattern::PProd(terms)
                }
            }
            MPattern::PDiv(box num, box den) => {
                MPattern::PDiv(box num.trivial_reduce(), box den.trivial_reduce())
            }
            x => x
        }
    }

    /// Tries to match a pattern to an expression, binding all variables in the pattern.
    /// Pseudo-example: `(const:a + var:b).bind(2x + 1) -> {const:a -> 1, var:b -> 2x}`
    pub fn bind(self, expr: MExpr) -> Result<(HashMap<u32, MExpr>, HashMap<u32, MExpr>), ()> {
        let mut const_res: HashMap<u32, MExpr> = HashMap::new();
        let mut var_res:   HashMap<u32, MExpr> = HashMap::new();
        match (self.trivial_reduce(), expr.trivial_reduce()) {
            (MPattern::PConst(n), other) => {
                if other.is_const() {
                    const_res.insert(n, other);
                } else {
                    return Err(());
                }
            }
            (MPattern::PVar(n), other) => {
                if other.is_const() {
                    return Err(());
                } else {
                    var_res.insert(n, other);
                }
            }
            (MPattern::PDiv(box a_pat, box b_pat), MExpr::Div(box a, box b)) => {
                let (const_res_a, var_res_a) = a_pat.bind(a)?;
                let (const_res_b, var_res_b) = b_pat.bind(b)?;
                merge(&mut const_res, const_res_a)?;
                merge(&mut const_res, const_res_b)?;
                merge(&mut var_res, var_res_a)?;
                merge(&mut var_res, var_res_b)?;
            }
            (MPattern::PSum(pterms), other) => {
                // println!("pterms = {:?}, other = {:?}", pterms, other);
                match other {
                    MExpr::Sum(terms) => {
                        let mut worked = false;
                        for (i_p, pattern) in pterms.clone().into_iter().enumerate() {
                            if worked { break }
                            for (i, elem) in terms.clone().into_iter().enumerate() {
                                // println!("i = {}, pattern = {:?}, elem = {:?}", i, pterms[0], elem);
                                if let Ok((const_res_f, var_res_f)) = pattern.clone().bind(elem.clone()) {
                                    // println!("Matched, const = {:?}, var = {:?}", const_res_f, var_res_f);

                                    let mut other_terms: Vec<MExpr> = terms[0..i].to_vec();
                                    for a in &terms[i+1..] {
                                        other_terms.push(a.clone());
                                    }

                                    let mut other_pterms: Vec<MPattern> = pterms[0..i_p].to_vec();
                                    for a in &pterms[i_p+1..] {
                                        other_pterms.push(a.clone());
                                    }

                                    // println!("rest pattern = {:?}, expr = {:?}", MPattern::PSum(other_pterms.clone()), MExpr::Sum(other_terms.clone()));
                                    if let Ok((const_res_r, var_res_r)) =
                                        MPattern::PSum(other_pterms).bind(MExpr::Sum(other_terms)) {
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
                            return Err(());
                        }
                    }
                    _ => return Err(())
                }
            }
            (MPattern::PProd(pterms), other) => {
                // println!("pterms = {:?}, other = {:?}", pterms, other);
                match other {
                    MExpr::Prod(terms) => {
                        let mut worked = false;
                        for (i_p, pattern) in pterms.clone().into_iter().enumerate() {
                            if worked { break }
                            for (i, elem) in terms.clone().into_iter().enumerate() {
                                // println!("i = {}, pattern = {:?}, elem = {:?}", i, pterms[0], elem);
                                if let Ok((const_res_f, var_res_f)) = pattern.clone().bind(elem.clone()) {
                                    // println!("Matched, const = {:?}, var = {:?}", const_res_f, var_res_f);

                                    let mut other_terms: Vec<MExpr> = terms[0..i].to_vec();
                                    for a in &terms[i+1..] {
                                        other_terms.push(a.clone());
                                    }

                                    let mut other_pterms: Vec<MPattern> = pterms[0..i_p].to_vec();
                                    for a in &pterms[i_p+1..] {
                                        other_pterms.push(a.clone());
                                    }

                                    if let Ok((const_res_r, var_res_r)) =
                                        MPattern::PProd(other_pterms).bind(MExpr::Prod(other_terms)) {
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
                            return Err(())
                        }
                    }
                    _ => return Err(())
                }
            }
            _ => return Err(())
        };
        Ok((const_res, var_res))
    }
}
#[test]
fn test_bind() {
    let pattern = MPattern::PProd(vec![MPattern::PConst(0), MPattern::PVar(0)]);
    let expr = MExpr::Prod(vec![MExpr::Var(0), MExpr::ConstNum(2)]);
    let bind = pattern.bind(expr);
    assert!(bind.is_ok());
    let bind = bind.unwrap();
    assert_eq!(bind.0.get(&0), Some(&MExpr::ConstNum(2)));
    assert_eq!(bind.1.get(&0), Some(&MExpr::Var(0)));

    let pattern = MPattern::PDiv(box MPattern::PConst(0), box MPattern::PVar(0));
    let expr = MExpr::Div(box MExpr::ConstNum(3), box MExpr::Prod(vec![ MExpr::ConstNum(2), MExpr::Var(21)] ));
    let bind = pattern.bind(expr);
    assert!(bind.is_ok());
    let bind = bind.unwrap();
    assert_eq!(bind.0.get(&0), Some(&MExpr::ConstNum(3)));
    assert_eq!(bind.1.get(&0), Some(&MExpr::Prod(vec![ MExpr::ConstNum(2), MExpr::Var(21)] )));

    let pattern = MPattern::PSum(vec![MPattern::PConst(0), MPattern::PVar(0)]);
    let expr = MExpr::Sum(vec![MExpr::Var(0), MExpr::ConstNum(2)]);
    let bind = pattern.bind(expr);
    assert!(bind.is_ok());
    let bind = bind.unwrap();
    assert_eq!(bind.0.get(&0), Some(&MExpr::ConstNum(2)));
    assert_eq!(bind.1.get(&0), Some(&MExpr::Var(0)));

}
