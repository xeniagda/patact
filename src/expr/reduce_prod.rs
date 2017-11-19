use expr::exprs::*;


/// Unfolds nested products, such that eg. `x*(y*z) -> x*y*z`
pub fn unfold_nested(terms: Vec<MExpr>) -> Vec<MExpr> {
    terms.into_iter()
        .flat_map(|term|
                  match term {
                      MExpr::Prod(terms) => terms,
                      _ => vec![term]
                  })
        .collect()
}

/// Combines the entire expression into a division, returning the denominator and numerator.
/// Gives `None` if there are no divisions in the expression.
pub fn unfold_division(terms: Vec<MExpr>) -> Option<(Vec<MExpr>, Vec<MExpr>)> {
    let mut nums = vec![];
    let mut dens = vec![];
    for term in terms {
        if let MExpr::Div(box num, box den) = term {
            nums.push(num);
            dens.push(den);
        } else {
            nums.push(term);
        }
    }
    if dens.is_empty() {
        None
    } else {
        Some((nums, dens))
    }
}

/// Multiplies together constants in an expression.
pub fn combine_constants(terms: Vec<MExpr>) -> Vec<MExpr> {
    let mut prod = 1;
    let mut other: Vec<MExpr> = vec![];

    for mut term in terms {
        match term {
            MExpr::ConstNum(x) => {
                prod *= x;
            }
            term => {
                other.push(term);
            }
        }
    }
    if prod == 0 {
        return vec![MExpr::ConstNum(prod)];
    } else if prod != 1 {
        other.push(MExpr::ConstNum(prod));
    }
    other
}

