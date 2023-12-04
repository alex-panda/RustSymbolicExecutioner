mod parser;
mod compiler;
mod symex;

use std::env;
use std::fs;
//use equation_solver::*;
use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
use crate::symex::{SymVar, SymExEngine};
use crate::parser::*;
use crate::parser::parser::parse_file;

//use parse_file;
use ParseResult::*;

//static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File name expected");
        return;
    }
    let filename = args[1].clone();


    let valid = compiler::compile_input(&filename);
        if valid {
           run_parser(filename);

        }

        else {
            println!("Could not compile");
        }
}

fn run_parser(filename: String) {

    let t = fs::read_to_string(filename).expect("Could not read");
    let text = t.as_str();
    match parse_file(text) {
        Okay(value, advance) => {
            let mut engine = Vec::new();
            println!("{:?}", value.execute(text, &mut engine, 0));
        },
        Error(error) => panic!("Error: {}", error),
        Panic(error) => panic!("Panic: {}", error),
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