#![feature(box_syntax, box_patterns)]

pub mod math_logic;
use math_logic::math_types::MExpr;
use std::io::{stdin, stdout, Result, Write};

fn main() {
    if let Err(e) = repl() {
        panic!(e);
    }
}

fn repl() -> Result<()> {
    let mut last: Option<MExpr> = None;

    loop {
        print!("\n> ");
        stdout().flush()?;

        let mut line = String::new();
        if stdin().read_line(&mut line)? == 0 {
            println!("");
            return Ok(())
        }
        line = line.trim().to_string();
        if line == "!" {
            match last.clone() {
                None => eprintln!("No last expression"),
                Some(expr) => {
                    println!("Last: {:?}", expr);
                    println!("Reduced: {:?}", expr.reduce(true));
                }
            }
        } else {
            match line.parse::<MExpr>() {
                Err((msg, idx)) => {
                    let idx = line.len() - idx;
                    eprintln!("{}", line);
                    eprintln!("{}^", " ".repeat(idx));
                    eprintln!("Error: {:?} at {}", msg, idx);
                }
                Ok(expr) => {
                    println!("   Expr:  {}", expr);
                    println!("Reduced:  {}", expr.clone().reduce(true));
                    last = Some(expr);
                }
            }
        }
    }
}
