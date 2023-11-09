mod parser;
mod compiler;
mod solver;

use std::env;
use equation_solver::*;
use crate::solver::SymVar;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File name expected");
        return;
    }
    println!("{}", &args[1]);

    let n = Equation::new("8");

    let mut v = SymVar {
        name: "v".to_string(),
        var0: "v + 5".to_string(),
        current_eq: n.unwrap(),
    };

    let valid = compiler::compile_input(&args[1]);
    if valid {
        println!("Hello World!");
        solver::solver_example().unwrap();
        solver::update_assignment(v, "v = x + 5".to_string());
    }

    else {
        println!("Could not compile");
    }
}