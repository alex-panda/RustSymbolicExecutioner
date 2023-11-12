use equation_solver::*;
use crate::solver::SymVar;
use smtlib::{backend::Z3Binary, Solver};

pub struct SymExEngine {
    pub pi: Solver<Z3Binary>,
    pub pi_str: String,
    pub sigma: Vec<SymVar>,
    pub path: u32,
}

pub fn demo_eval() {
    let n = Equation::new("x*((2 + 4) + 5)");

    let mut eq1 = n.unwrap();
    Equation::set_value(&mut eq1, "x", 8.into());

    let t = eq1.evaluate();

    println!("{t:?}");
}

pub fn test_symex(mut e: SymExEngine) -> SymExEngine {
    e = new_variable(e, "x".to_string());
    e = new_variable(e, "y".to_string());
    e = update_symvar_value(e, "y + 4".to_string(), "x".to_string());
    e = update_symvar_value(e, "2*x".to_string(), "y".to_string());

    return e;
}

//Symbolic let
pub fn new_variable(mut e: SymExEngine, var: String) -> SymExEngine {
    let v = SymVar::new(var, "i32".to_string());
    e.sigma.push(v);
    return e;
}


pub fn update_symvar_value(mut e: SymExEngine, mut stmt_rs: String, stmt_ls: String) -> SymExEngine {
    let mut i = 0;
    while i < e.sigma.len() {
        if stmt_rs.contains(&e.sigma[i].name) {
            let s = format!("({})", e.sigma[i].var0);
            stmt_rs = stmt_rs.replace(&e.sigma[i].name, &s);
        }
        i = i + 1;
    }
    let mut j = 0;
    let mut found = false;
    while j < e.sigma.len() {
        if stmt_ls.contains(&e.sigma[j].name) {
            found = true;
            e.sigma[j].prev = e.sigma[j].var0.clone();
            e.sigma[j].var0 = stmt_rs.clone();
        }
        j = j + 1;
    }
    if !found {
        panic!("assignment to uninitialized variable attempted");
    }

    return e;
}


impl SymExEngine {
    pub fn to_string(&self) -> String {
        let mut i = 0;
        let mut s: String = "".to_string();
        while i < self.sigma.len() {
            s = s + "\t" + &self.sigma[i].to_string() + "\n";
            i = i + 1;
        }
        format!("path: {}\npi: {}\nsigma: {}", &self.path, &self.pi_str, s)
    }
}