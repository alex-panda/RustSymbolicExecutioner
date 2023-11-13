mod parser;
mod compiler;
mod solver;

use std::env;
//use equation_solver::*;
use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
use crate::solver::{SymVar, SymExEngine};

static PATH_TO_SOLVER:&str = "z3\\bin\\z3";
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut num_paths = 1;
    if args.len() != 2 {
        println!("File name expected");
        return;
    }
    println!("{}", &args[1]);

    let init_engine = || -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = SymExEngine {
            pi: Solver::new(Z3Binary::new(PATH_TO_SOLVER)?)?,
            pi_str: "true".to_string(),
            sigma: Vec::new(),
            path: num_paths,
        };

        let valid = compiler::compile_input(&args[1]);
        if valid {
            //println!("Hello World!");
            engine = solver::test_symex(engine);
            println!("{}", engine.to_string());
            //solver::solver_example(&mut engine.pi).unwrap();
            //solver::demo_eval();
        }

        else {
            println!("Could not compile");
        }
        Ok(())
    };

    if let Err(_err) = init_engine() {
        println!("Failed to initialize symbolic execution engine.");
    }
}
