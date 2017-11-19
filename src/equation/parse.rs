use equation::equation::MEquation;
use equation::eq_pattern::EPattern;
use expr::exprs::MExpr;
use expr::expr_pattern::MPattern;
use utils::find_depth0;

use std::str::FromStr;


impl FromStr for MEquation {
    type Err = (String, usize); // (msg, length from end)

    fn from_str(input: &str) -> Result<MEquation, Self::Err> {
        let equal_signs = find_depth0(input, |c| c == '=', '(', ')');
        if equal_signs.len() == 1 {
            let lhs = input[..equal_signs[0]].parse::<MExpr>()?;
            let rhs = input[equal_signs[0]+1..].parse::<MExpr>()?;
            Ok(MEquation::Equal(lhs, rhs))
        } else {
            Err(("No '=' sign".to_string(), 0))
        }
    }
}


impl FromStr for EPattern {
    type Err = (String, usize); // (msg, length from end)

    fn from_str(input: &str) -> Result<EPattern, Self::Err> {
        let equal_signs = find_depth0(input, |c| c == '=', '(', ')');
        if equal_signs.len() == 1 {
            let lhs = input[..equal_signs[0]].parse::<MPattern>()?;
            let rhs = input[equal_signs[0]+1..].parse::<MPattern>()?;
            Ok(EPattern::PEq(lhs, rhs))
        } else {
            Err(("No '=' sign".to_string(), 0))
        }
    }
}
