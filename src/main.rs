mod parser;
mod compiler;
mod symex;

use std::env;
//use equation_solver::*;
use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
use crate::symex::{SymVar, SymExEngine};

static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File name expected");
        return;
    }
    println!("{}", &args[1]);

    let valid = compiler::compile_input(&args[1]);
        if valid {
            println!("Hello World!");
        }

        else {
            println!("Could not compile");
        }
}


#[cfg(test)] 
mod test {

    use rsmt2::*;

    #[test]
    fn rsmt2_test() -> Result<(), Box<dyn std::error::Error>> {
        let parser = ();
        let conf = SmtConf::z3("z3\\bin\\z3.exe");
        let mut solver = conf.spawn(parser).unwrap();

        solver.declare_const("n", "Int")?;
        solver.declare_const("m", "Int")?;
        solver.assert("(= 4 5)")?;

        let is_sat = solver.check_sat()?;
        assert!(is_sat);
        Ok(())
    }
}