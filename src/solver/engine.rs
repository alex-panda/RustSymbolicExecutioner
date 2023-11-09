use equation_solver::*;

pub struct SymVar {
    pub name: String,
    pub var0: String,
    pub current_eq: Equation,
}

pub fn update_assignment(var: SymVar, stmt: String) {
    let n = Equation::new("8*((2 + 4) + 5)");

    let eq1 = n.unwrap();

    let t = eq1.evaluate();

    println!("{t:?}");
}

