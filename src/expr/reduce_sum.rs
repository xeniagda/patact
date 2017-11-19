use expr::exprs::*;


/// Unfolds nested sums, such that eg. `x+(y+z) -> x+y+z`
pub fn unfold_nested(terms: Vec<MExpr>) -> Vec<MExpr> {
    terms.into_iter()
        .flat_map(|term|
                  match term {
                      MExpr::Sum(terms) => terms,
                      _ => vec![term]
                  })
        .collect()
}

/// Sums together constants in an expression.
pub fn combine_constants(terms: Vec<MExpr>) -> Vec<MExpr> {
    let mut sum = 0;
    let mut other: Vec<MExpr> = vec![];

    for mut term in terms {
        match term {
            MExpr::ConstNum(x) => {
                sum += x;
            }
            term => {
                other.push(term);
            }
        }
    }
    if sum != 0 {
        other.push(MExpr::ConstNum(sum));
    }
    other
}

