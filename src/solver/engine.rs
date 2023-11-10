use equation_solver::*;
use smtlib::{backend::Z3Binary, Solver};
pub struct SymVar {
    pub name: String,
    pub var0: String,
    pub prev: String,
}

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

