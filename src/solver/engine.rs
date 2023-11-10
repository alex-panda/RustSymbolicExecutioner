use equation_solver::*;
use crate::solver::SymVar;
use smtlib::{backend::Z3Binary, Solver};

pub struct SymExEngine {
    pub pi: Solver<Z3Binary>,
    pub pi_str: String,
    pub sigma: Vec<SymVar>,
    pub path: String,
}

pub fn demo_eval() {
    let n = Equation::new("x*((2 + 4) + 5)");

    let mut eq1 = n.unwrap();
    Equation::set_value(&mut eq1, "x", 8.into());

    let t = eq1.evaluate();

    println!("{t:?}");
}

pub fn test_symex(mut e: SymExEngine) {
    let x = SymVar::new("x".to_string());
    let y = SymVar::new("y".to_string());
    e.sigma.push(x);
    e.sigma.push(y);
    println!("{}", e.sigma[0].to_string());
    update_symvar_value(e, "y + 5".to_string(), "x".to_string());
}

pub fn update_symvar_value(mut e: SymExEngine, mut stmt_rs: String, stmt_ls: String) {
    let mut i = 0;
    while i < e.sigma.len() {
        println!("{}", e.sigma[i].name);
        if stmt_rs.contains(&e.sigma[i].name) {
            let s = format!("({})", e.sigma[i].var0);
            stmt_rs = stmt_rs.replace(&e.sigma[i].name, &s);
        }
        i = i + 1;
    }
    let mut j = 0;
    while j < e.sigma.len() {
        if stmt_ls.contains(&e.sigma[j].name) {
            e.sigma[j].prev = e.sigma[j].var0.clone();
            e.sigma[j].var0 = stmt_rs.clone();
            println!("{}", &e.sigma[j].to_string());
        }
        j = j + 1;
    }
}