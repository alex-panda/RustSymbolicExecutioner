use equation_solver::*;
use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
use crate::solver::{SymVar};


pub struct SymExEngine {
    pub pi: Solver<Z3Binary>,
    pub pi_str: String,
    pub sigma: Vec<SymVar>,
    pub path: u32,
}

//creates symvar from function header
//ie (mut var_name: var_type)
pub fn new_variable(mut e: SymExEngine, var_name: String, var_type: String) -> SymExEngine {
    let v = SymVar::new(var_name, var_type);
    e.sigma.push(v);
    return e;
}

//creates symvar from initialization
//ie let var_name: var_type = assign;
pub fn new_variable_assign(mut e: SymExEngine, var_name: String, var_type: String, assign: String) -> SymExEngine {
    let v = SymVar::new_assign(var_name, var_type, assign);
    e.sigma.push(v);
    return e;
}

//pub fn new_assertion(mut vec: Vec<SymExEngine>, assertion: String) {
    //TODO create a path where the assertion is true and one where it's false
//}

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

#[cfg(test)]
mod tests {
    use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
    use crate::solver::*;
    use equation_solver::*;
    static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

    #[test]
    fn demo_engine() {
        let init_engine = || -> Result<(), Box<dyn std::error::Error>> {
            let mut e = SymExEngine {
                pi: Solver::new(Z3Binary::new(PATH_TO_SOLVER)?)?,
                pi_str: "true".to_string(),
                sigma: Vec::new(),
                path: 1,
            };
            //demonstration of updating sigma using predetermined variables and assignments

            e = new_variable(e, "x".to_string(), "i32".to_string());
            e = new_variable(e, "y".to_string(), "u64".to_string());
            e = update_symvar_value(e, "y + 4".to_string(), "x".to_string());
            e = update_symvar_value(e, "2*x".to_string(), "y".to_string());

            println!("{}", e.to_string());

            Ok(())
        };
    
        if let Err(_err) = init_engine() {
            println!("Failed to initialize symbolic execution engine.");
        }
    }

    #[test]
    fn demo_eval() {
        let n = Equation::new("x*((2 + 4) + 5)");
    
        let mut eq1 = n.unwrap();
        Equation::set_value(&mut eq1, "x", 8.into());
    
        let t = eq1.evaluate();
    
        println!("{t:?}");
    }
}