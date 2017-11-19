
use expr::exprs::MExpr;
use expr::expr_pattern::MPattern;
use std::str::FromStr;
use utils::find_depth0;


impl FromStr for MExpr {
    type Err = (String, usize); // (msg, length from end)

    fn from_str(input: &str) -> Result<MExpr, Self::Err> {
        let input = input.trim();

        if input.chars().next() == Some('(') {
            let mut is_parenthesised = true;

            let mut depth = 0u16;
            for (i, ch) in input.chars().enumerate() {
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    if depth == 0 {
                        return Err(("No matching paren".to_string(), input.len() - i - 1));
                    }
                    depth -= 1;
                } else if depth == 0 {
                    is_parenthesised = false;
                    break;
                }
            }
            if depth != 0 {
                return Err(("Mismatched parethesis".to_string(), 0));
            }
            if is_parenthesised {
                let mut x = input.to_string();
                x.pop();
                x.remove(0);
                return Self::from_str(&x);
            }
        }

        // Is addition?
        let mut pluses: Vec<usize> = 
            find_depth0(&input, |ch| ch == '+' || ch == '-', '(', ')')
                .into_iter()
                .filter(|x| x != &0usize)
                .collect();

        if !pluses.is_empty() {
            pluses.push(input.len());
            let mut terms = vec![];

            let mut term_start = 0;

            for term_end in pluses {
                let term = &input[term_start..term_end];
                terms.push(term.parse::<MExpr>()?);

                if input.chars().nth(term_end) == Some('-') {
                    term_start = term_end;
                } else {
                    term_start = term_end + 1;
                }
            }
            return Ok(MExpr::Sum(terms));
        }

        // Is multiplication?
        let mut times = find_depth0(&input, |ch| ch == '*', '(', ')');
        if !times.is_empty() {
            times.push(input.len());
            let mut factors = vec![];

            let mut factor_start = 0;

            for factor_end in times {
                let factor = &input[factor_start..factor_end];

                factors.push(factor.parse::<MExpr>()?);

                factor_start = factor_end + 1;
            }
            return Ok(MExpr::Prod(factors));
        }

        // Is division
        let divs = find_depth0(&input, |ch| ch == '/', '(', ')');
        if !divs.is_empty() {
            let num = input[..divs[0]].parse::<MExpr>()?;
            let den = input[divs[0] + 1..].parse::<MExpr>()?;
            return Ok(MExpr::Div(box num, box den));
        }
        
        // Is negation?
        if input.chars().next() == Some('-') {
            let expr: MExpr = input[1..].parse()?;
            return Ok(MExpr::Prod(vec![MExpr::ConstNum(-1), expr]));
        }


        // Is constant?
        if let Some(ch) = input.chars().next() {
            if ch.is_uppercase() && input.chars().count() == 1 {
                return Ok(MExpr::ConstVar(
                    (input.bytes().next().unwrap() - b'A') as u32,
                ));
            }
        }
        // Is variable?
        if let Some(ch) = input.chars().next() {
            if ch.is_lowercase() && input.chars().count() == 1 {
                return Ok(MExpr::Var((input.bytes().next().unwrap() - b'a') as u32));
            }
        }
        // Is number?
        if let Ok(num) = input.parse::<i64>() {
            return Ok(MExpr::ConstNum(num));
        }

        Err(("Unknown operator".to_string(), input.len()))
    }
}
impl FromStr for MPattern {
    type Err = (String, usize); // (msg, length from end)

    fn from_str(input: &str) -> Result<MPattern, Self::Err> {
        let input = input.trim();

        if input.chars().next() == Some('(') {
            let mut is_parenthesised = true;

            let mut depth = 0u16;
            for (i, ch) in input.chars().enumerate() {
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    if depth == 0 {
                        return Err(("No matching paren".to_string(), input.len() - i - 1));
                    }
                    depth -= 1;
                } else if depth == 0 {
                    is_parenthesised = false;
                    break;
                }
            }
            if depth != 0 {
                return Err(("Mismatched parethesis".to_string(), 0));
            }
            if is_parenthesised {
                let mut x = input.to_string();
                x.pop();
                x.remove(0);
                return Self::from_str(&x);
            }
        }

        // Is addition?
        let mut pluses: Vec<usize> = 
            find_depth0(&input, |ch| ch == '+' || ch == '-', '(', ')')
                .into_iter()
                .filter(|x| x != &0usize)
                .collect();

        if !pluses.is_empty() {
            pluses.push(input.len());
            let mut terms = vec![];

            let mut term_start = 0;

            for term_end in pluses {
                let term = &input[term_start..term_end];
                terms.push(term.parse::<MPattern>()?);

                if input.chars().nth(term_end) == Some('-') {
                    term_start = term_end;
                } else {
                    term_start = term_end + 1;
                }
            }
            return Ok(MPattern::PSum(terms));
        }

        // Is multiplication?
        let mut times = find_depth0(&input, |ch| ch == '*', '(', ')');
        if !times.is_empty() {
            times.push(input.len());
            let mut factors = vec![];

            let mut factor_start = 0;

            for factor_end in times {
                let factor = &input[factor_start..factor_end];

                factors.push(factor.parse::<MPattern>()?);

                factor_start = factor_end + 1;
            }
            return Ok(MPattern::PProd(factors));
        }

        // Is division
        let divs = find_depth0(&input, |ch| ch == '/', '(', ')');
        if !divs.is_empty() {
            let num = input[..divs[0]].parse::<MPattern>()?;
            let den = input[divs[0] + 1..].parse::<MPattern>()?;
            return Ok(MPattern::PDiv(box num, box den));
        }


        // Is constant?
        if let Some(ch) = input.chars().next() {
            if ch.is_uppercase() && input.chars().count() == 1 {
                return Ok(MPattern::PConst(
                    (input.bytes().next().unwrap() - b'A') as u32,
                ));
            }
        }

        // Is variable?
        if let Some(ch) = input.chars().next() {
            if ch.is_lowercase() && input.chars().count() == 1 {
                return Ok(MPattern::PVar((input.bytes().next().unwrap() - b'a') as u32));
            }
        }

        Err(("Unknown operator".to_string(), input.len()))
    }
}

#[test]
fn test_parse_mexpr() {

    assert_eq!("(5)".parse::<MExpr>(), Ok(MExpr::ConstNum(5)));
    assert_eq!(
        "5 + 2 + 3".parse::<MExpr>(),
        Ok(MExpr::Sum(vec![
            MExpr::ConstNum(5),
            MExpr::ConstNum(2),
            MExpr::ConstNum(3),
        ]))
    );
    assert_eq!(
        "a + A".parse::<MExpr>(),
        Ok(MExpr::Sum(vec![MExpr::Var(0), MExpr::ConstVar(0)]))
    );
    assert_eq!(
        "3 * a".parse::<MExpr>(),
        Ok(MExpr::Prod(vec![MExpr::ConstNum(3), MExpr::Var(0)]))
    );
    assert_eq!(
        "3 * 5 * a".parse::<MExpr>(),
        Ok(MExpr::Prod(
            vec![MExpr::ConstNum(3), MExpr::ConstNum(5), MExpr::Var(0)]
        ))
    );
    assert_eq!("  ( 321 )".parse::<MExpr>(), Ok(MExpr::ConstNum(321)));
    assert_eq!("A".parse::<MExpr>(), Ok(MExpr::ConstVar(0)));
    assert_eq!("c".parse::<MExpr>(), Ok(MExpr::Var(2)));
}

#[test]
fn test_parse_epattern() {
    assert_eq!("A".parse::<MPattern>(), Ok(MPattern::PConst(0)));
    assert_eq!("a".parse::<MPattern>(), Ok(MPattern::PVar(0)));
    assert_eq!("a + X".parse::<MPattern>(), Ok(MPattern::PSum(vec![MPattern::PVar(0), MPattern::PConst(23)])));
}
