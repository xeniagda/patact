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
        if input.len() == 2 {
            let action = input.chars().nth(0).unwrap();
            let arg = input[1..].parse::<MPattern>()?;

            match (action, arg) {
                ('+', MPattern::Const(x)) => Ok(Action::AddC(x)),
                ('-', MPattern::Const(x)) => Ok(Action::SubC(x)),
                ('*', MPattern::Const(x)) => Ok(Action::MulC(x)),
                ('/', MPattern::Const(x)) => Ok(Action::DivC(x)),
                ('+', MPattern::Var(x))   => Ok(Action::AddV(x)),
                ('-', MPattern::Var(x))   => Ok(Action::SubV(x)),
                ('*', MPattern::Var(x))   => Ok(Action::MulV(x)),
                ('/', MPattern::Var(x))   => Ok(Action::DivV(x)),
                _                         => Err(("Couldn't read!".to_string(), 0))
            }
        } else if input == "done" {
            Ok(Action::DoNothing())
        } else {
            Err(("Couldn't read!".to_string(), 0))
        }
    }
}

#[test]
fn test_patact() {
    let inp = "a + A = B > -A";
    let parsed = inp.parse::<PatternAction>();
    assert!(parsed.is_ok());
    assert_eq!(parsed.unwrap(),
               PatternAction { pattern: EPattern::PEq(MPattern::Sum(vec![MPattern::Var(0), MPattern::Const(0)]), MPattern::Const(1)), action: Action::SubC(0) }
              )
}
