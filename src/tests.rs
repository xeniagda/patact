use expr::exprs::MExpr;
use expr::expr_pattern::MPattern;

macro_rules! assert_unwrap_res {
    ( $x:expr ) => {
        {
            match $x {
                Ok(a) => a,
                Err(e) => { assert!(false, format!("{:?} is not Ok", e)); $x.unwrap() }
            }
        }
    }
}

macro_rules! assert_unwrap_option {
    ( $x:expr ) => {
        {
            let e = $x;
            assert!(e.is_some());
            e.unwrap()
        }
    }
}

#[test]
fn test_basic_reduction_and_parsing() {
    let expr = assert_unwrap_res!("1 + 2 + 3".parse::<MExpr>());


    assert_eq!(
        expr.reduce(true),
        MExpr::ConstNum(6)
          );


    let expr = "2 * 3 + 2".parse::<MExpr>();

    assert!(expr.is_ok());
    let expr = expr.unwrap();

    assert_eq!(
        expr.reduce(true),
        MExpr::ConstNum(8)
          );


    let expr = "(62 * a * a) / (2 * a)".parse::<MExpr>();

    assert!(expr.is_ok());
    let expr = expr.unwrap();

    assert_eq!(
        expr.reduce(true),
        MExpr::Prod(vec![MExpr::Var(0), MExpr::ConstNum(31)])
          );


    let expr = "(2 * a + 4) / (a + 2)".parse::<MExpr>();

    assert!(expr.is_ok());
    let expr = expr.unwrap();

    assert_eq!(
        expr.reduce(true),
        MExpr::ConstNum(2)
          );
}

#[test]
fn test_simple_matching() {
    let expr = assert_unwrap_res!("2 * x + 2 / x".parse::<MExpr>());
    let pattern = assert_unwrap_res!("a + b".parse::<MPattern>());

    let pmatch = assert_unwrap_option!(pattern.bind(expr));

    assert!(pmatch.0.is_empty());
    assert_eq!(
        pmatch.1.get(&0),
        Some(&MExpr::Prod(vec![MExpr::ConstNum(2), MExpr::Var(23)]))
        );
    assert_eq!(
        pmatch.1.get(&1),
        Some(&MExpr::Div(box MExpr::ConstNum(2), box MExpr::Var(23)))
        );


    let expr = assert_unwrap_res!("5 * y - 7 + x".parse::<MExpr>()).reduce(true);
    let pattern = assert_unwrap_res!("a + b + A".parse::<MPattern>());

    let pmatch = assert_unwrap_option!(pattern.bind(expr));

    assert_eq!(
        pmatch.0.get(&0),
        Some(&MExpr::ConstNum(-7))
        );
    assert_eq!(
        pmatch.1.get(&0),
        Some(&MExpr::Prod(vec![MExpr::Var(24), MExpr::ConstNum(5)]))
        );
    assert_eq!(
        pmatch.1.get(&1),
        Some(&MExpr::Var(23))
        );
}

#[test]
fn test_matching_same_var() {
    let expr = assert_unwrap_res!("A * (x + 2) + A / (x + 2)".parse::<MExpr>());
    let pattern = assert_unwrap_res!("A * a + A / b".parse::<MPattern>());

    let pmatch = assert_unwrap_option!(pattern.bind(expr));

    assert_eq!(
        pmatch.0.get(&0),
        Some(&MExpr::ConstVar(0))
        );
    assert_eq!(
        pmatch.1.get(&1),
        Some(&MExpr::Sum(vec![ MExpr::Var(23), MExpr::ConstNum(2) ]))
        );
}
