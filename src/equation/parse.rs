use equation::equation::MEquation;
use equation::eq_pattern::EPattern;
use equation::action::{PatternAction, Action};
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
        let input = input.trim();
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

impl FromStr for PatternAction {
    type Err = (String, usize); // (msg, length from end)

    fn from_str(input: &str) -> Result<PatternAction, Self::Err> {
        let input = input.trim();
        if let Some(div) = find_depth0(input, |c| c == '>', '(', ')').into_iter().nth(0) {
            let pattern = input[..div].parse::<EPattern>()?;
            let action = input[div+1..].parse::<Action>()?;
            Ok(PatternAction{pattern, action})
        } else {
            Err(("No '=' sign".to_string(), 0))
        }
    }
}

impl FromStr for Action {
    type Err = (String, usize); // (msg, length from end)

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        let input = input.trim();
        if let Some(space) = find_depth0(input, |c| c == ' ', '(', ')').into_iter().nth(0) {
            let action = &input[..space];
            let arg = input[space+1..].parse::<MPattern>()?;

            match (action, arg) {
                ("add", MPattern::PConst(x)) => Ok(Action::AddC(x)),
                ("sub", MPattern::PConst(x)) => Ok(Action::SubC(x)),
                ("mul", MPattern::PConst(x)) => Ok(Action::MulC(x)),
                ("div", MPattern::PConst(x)) => Ok(Action::DivC(x)),
                ("add", MPattern::PVar(x))   => Ok(Action::AddV(x)),
                ("sub", MPattern::PVar(x))   => Ok(Action::SubV(x)),
                ("mul", MPattern::PVar(x))   => Ok(Action::MulV(x)),
                ("div", MPattern::PVar(x))   => Ok(Action::DivV(x)),
                _ => Err(("Couldn't read!".to_string(), 0))
            }
        } else {
            Err(("Couldn't read!".to_string(), 0))
        }
    }
}

#[test]
fn test_patact() {
    let inp = "a + A = B > sub A";
    let parsed = inp.parse::<PatternAction>();
    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(),
               PatternAction { pattern: EPattern::PEq(MPattern::PSum(vec![MPattern::PVar(0), MPattern::PConst(0)]), MPattern::PConst(1)), action: Action::SubC(0) }
              )
}
