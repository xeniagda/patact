#![feature(box_syntax, box_patterns)]

pub mod math_logic;
use math_logic::math_types::MExpr::*;


fn main() {
    let mut exprs = vec! [
        Sum(vec![
            ConstVar(0),
            ConstNum(10),
            Prod(vec![ Var(0), ConstNum(10) ])
        ]),
        Exp(
            box Sum(vec![ ConstVar(0), ConstNum(10) ]),
            box ConstNum(2)
           ),
        ConstVar(0),
        Sum(vec![
            ConstVar(0),
            ConstNum(10)
        ]),
        Prod(vec![
             Sum(vec![ConstNum(5), ConstNum(3)]),
             ConstNum(10)
        ]),
        Sum(vec![
            ConstVar(0),
            ConstNum(11)
        ]),
        Sum(vec![
            ConstNum(3),
            ConstVar(0),
            ConstNum(11)
        ]),
        Sum(vec![
            Prod(vec![ Var(0), ConstNum(10) ]),
            Prod(vec![ Var(0), ConstNum(5) ])
        ]),
    ];



    for element in &exprs {
        println!("Element: {}", element);
    }

    exprs.sort();

    for element in &exprs {
        println!("Sorted: {}", element);
    }

    for element in &exprs {
        println!("{} -> {}", element, element.clone().reduce());
    }
}
