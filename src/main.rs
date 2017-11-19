#![feature(box_syntax, box_patterns)]

pub mod expr;
pub mod equation;
pub mod utils;

use expr::exprs::MExpr;
use expr::expr_pattern::MPattern;
use equation::equation::MEquation;
use equation::eq_pattern::EPattern;
use equation::action::PatternAction;
use std::io::{stdin, stdout, Result, Write};

fn main() {
    print!("Equation? [Y/n] ");
    stdout().flush();
    let mut line = String::new();
    if let Err(e) = stdin().read_line(&mut line) {
        panic!(e);
    }
    if line.trim() == "n" {
        if let Err(e) = repl_expr() {
            panic!(e);
        }
    } else {
        if let Err(e) = repl_eq() {
            panic!(e);
        }
    }


}

fn repl_eq() -> Result<()> {
    let mut last: Option<MEquation> = None;

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
                    println!("Reduced: {:?}", expr.reduce());
                }
            }
        } else {
            if line.starts_with(":match") || line.starts_with(":Match") {
                if line.chars().nth(1) == Some('m') {
                    last = last.map(|e| e.reduce());
                }
                let line = line[6..].trim();
                match (last.clone(), line.parse::<EPattern>()) {
                    (_, Err((msg, idx))) => {
                        let idx = line.len() - idx;
                        eprintln!("{}", line);
                        eprintln!("{}^", " ".repeat(idx));
                        eprintln!("Error: {:?} at {}", msg, idx);
                    }
                    (Some(last), Ok(pattern)) => {
                        match pattern.bind(last) {
                            Ok((mut consts, mut vars)) => {
                                println!("Consts:");
                                consts.drain().for_each(|(k, v)| println!("\t{} = {}", MPattern::PConst(k), v));
                                println!("Vars:");
                                vars.drain().for_each(|(k, v)| println!("\t{} = {}", MPattern::PVar(k), v));
                            }
                            Err(_) => {
                                eprintln!("Couldn't bind!");
                            }
                        }
                    }
                    _ => {
                        eprintln!("No last expression!");
                    }
                }
            } else if line.starts_with(":do") || line.starts_with(":Do") {
                if line.chars().nth(1) == Some('D') {
                    last = last.map(|e| e.reduce());
                }
                let line = line[4..].trim();
                match (last.clone(), line.parse::<PatternAction>()) {
                    (_, Err((msg, idx))) => {
                        let idx = line.len() - idx;
                        eprintln!("{}", line);
                        eprintln!("{}^", " ".repeat(idx));
                        eprintln!("Error: {:?} at {}", msg, idx);
                    }
                    (Some(last_), Ok(patact)) => {
                        match patact.apply(last_) {
                            Ok(eq) => {
                                println!("    Res: {}", eq);
                                println!("Reduced: {}", eq.clone().reduce());
                                last = Some(eq);
                            }
                            Err(_) => {
                                eprintln!("Couldn't Apply!");
                            }
                        }
                    }
                    _ => {
                        eprintln!("No last expression!");
                    }
                }
            } else {
                match line.parse::<MEquation>() {
                    Err((msg, idx)) => {
                        let idx = line.len() - idx;
                        eprintln!("{}", line);
                        eprintln!("{}^", " ".repeat(idx));
                        eprintln!("Error: {:?} at {}", msg, idx);
                    }
                    Ok(expr) => {
                        println!("   Expr: {}", expr);
                        println!("Reduced: {}", expr.clone().reduce());
                        last = Some(expr);
                    }
                }
            }
        }
    }
}

fn repl_expr() -> Result<()> {
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
            if line.starts_with(":match") || line.starts_with(":Match") {
                if line.chars().nth(1) == Some('m') {
                    last = last.map(|e| e.reduce(true));
                }
                let line = line[6..].trim();
                match (last.clone(), line.parse::<MPattern>()) {
                    (_, Err((msg, idx))) => {
                        let idx = line.len() - idx;
                        eprintln!("{}", line);
                        eprintln!("{}^", " ".repeat(idx));
                        eprintln!("Error: {:?} at {}", msg, idx);
                    }
                    (Some(last), Ok(pattern)) => {
                        match pattern.bind(last) {
                            Ok((mut consts, mut vars)) => {
                                println!("Consts:");
                                consts.drain().for_each(|(k, v)| println!("\t{} = {}", MPattern::PConst(k), v));
                                println!("Vars:");
                                vars.drain().for_each(|(k, v)| println!("\t{} = {}", MPattern::PVar(k), v));
                            }
                            Err(_) => {
                                eprintln!("Couldn't bind!");
                            }
                        }
                    }
                    _ => {
                        eprintln!("No last expression!");
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
                        println!("   Expr: {}", expr);
                        println!("Reduced: {}", expr.clone().reduce(true));
                        last = Some(expr);
                    }
                }
            }
        }
    }
}
