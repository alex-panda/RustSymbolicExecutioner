use equation_solver::*;
use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
use crate::symex::{SymVar, SymSolver, SymExEngine};
use std::ops::Deref;

pub fn new_engine(engines: &mut Vec<SymExEngine>) -> usize {
    let id = engines.len();

    let mut init_engine = || -> Result<(), Box<dyn std::error::Error>> {
        let mut e = SymExEngine {
            pi: SymSolver::new(),
            sigma: Vec::new(),
            path: engines.len() as u32,
            reached_symex: false,
        };
        engines.push(e);
        Ok(())
    };

    if let Err(_err) = init_engine() {
        println!("Failed to initialize symbolic execution engine.");
    }

    id
}

pub fn clone_engine(engines: &mut Vec<SymExEngine>, path: usize) -> usize {
    let new_id = engines.len();

    let mut init_engine = || -> Result<(), Box<dyn std::error::Error>> {
        let mut e = SymExEngine {
            pi: SymSolver::copy_solver(&engines[path].pi),
            sigma: (*engines[path].sigma).to_vec(),
            path: engines.len() as u32,
            reached_symex: engines[path].reached_symex.clone(),
        };
        engines.push(e);
        Ok(())
    };

    if let Err(_err) = init_engine() {
        println!("Failed to initialize symbolic execution engine.");
    }

    new_id
}


pub fn new_assert(e: &mut Vec<SymExEngine>, path: usize, assert: String, lisp: String) -> usize {
    //println!("new assert");
    let l = e.len();
    clone_engine(e, path);
    //println!("asserting {}", assert.clone());
    //println!("asserting l {}", lisp.clone());
    
    e[l].new_assertion(assert.clone(), lisp.clone());
    let neg_assert = "!".to_owned() + &assert.clone();
    let neg_lisp = format!("(not {})", lisp.clone());
    e[path].new_assertion(neg_assert.clone(), neg_lisp);
    l
}

#[cfg(test)]
mod tests {
    use crate::symex::*;
    #[test]
    pub fn start() {
        let mut engines: Vec<SymExEngine> = Vec::new();
        pather::new_engine(&mut engines);
        println!("{}", engines[0].to_string());
    }

    /*#[test]
    pub fn test_new_assert() -> Result<(), Box<dyn std::error::Error>> {
        let mut engines: Vec<SymExEngine> = Vec::new();
        pather::new_engine(&mut engines);
        pather::new_engine(&mut engines);
        engines[1].new_variable("x".to_string(), "i32".to_string());
        engines[1].new_variable_assign("y".to_string(), "u64".to_string(), "5 + 6".to_string());
        engines[1].assign_symvar_value("x + 4".to_string(), "x".to_string());
        engines[1].assign_symvar_value("x * 2".to_string(), "y".to_string());
        new_assert(&mut engines, 1, "y == 18".to_string());
    
        let is_sat = engines[2].pi.s.check_sat()?;
        println!("{}", engines[2].to_string());
        assert!(is_sat);
        Ok(())
    }*/

}