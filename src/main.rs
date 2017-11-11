#![feature(box_syntax, box_patterns)]

mod math_types;
use math_types::MExpr::*;


fn main() {
    let mut exprs = [
        Sum(vec![
                   ConstVar(0),
                   ConstNum(10),
                   Prod(vec![ Var(0), ConstNum(10) ])
        ]),
        Exp(
            box Sum(vec![ ConstVar(0), ConstNum(10) ]),
            box ConstNum(2)),
        ConstVar(0),
        Sum(vec![ 
                   ConstVar(0),
                   ConstNum(10) 
        ]),
        Sum(vec![ 
                   ConstVar(0),
                   ConstNum(11) 
        ]),
    ];
    


    for element in &exprs {
        println!("Element: {}", element);
    }

    exprs.sort();

    for element in &exprs {
        println!("Sorted: {}", element);
    }
}
