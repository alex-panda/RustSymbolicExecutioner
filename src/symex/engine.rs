use equation_solver::*;
use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
use crate::symex::{SymVar, SymSolver};


pub struct SymExEngine {
    pub pi: SymSolver,
    pub sigma: Vec<SymVar>,
    pub path: u32,
}



pub fn eval(stmt_rs: String) -> String {
    let mut new = "".to_string();
    let n = Equation::new(stmt_rs.clone());
    let mut eq = n.unwrap();

    let wrap_result = eq.evaluate();
    
    let eval = match wrap_result {
        Ok(f64) => new = wrap_result.unwrap().to_string(),
        Err(EquationError) => new = stmt_rs.clone(),
    };

    return new;
}

pub fn new_assertion(assert: String) {

}

impl SymExEngine {
    pub fn to_string(&self) -> String {
        let mut i = 0;
        let mut s: String = "".to_string();
        while i < self.sigma.len() {
            s = s + "\t" + &self.sigma[i].to_string() + "\n";
            i = i + 1;
        }
        format!("path: {}\npi: {}\nsigma: {}", &self.path, &self.pi.to_string(), s)
    }

    //creates symvar from function header
//ie (mut var_name: var_type)
pub fn new_variable(&mut self, var_name: String, var_type: String) {
    let v = SymVar::new(var_name.clone(), var_type);
    self.sigma.push(v);
    self.pi.add_int(var_name.clone());
}

//creates symvar from initialization
//ie let var_name: var_type = assign;
pub fn new_variable_assign(&mut self, var_name: String, var_type: String, assign: String) {
    let v = SymVar::new_assign(var_name.clone(), var_type, eval(assign));
    self.sigma.push(v);
    self.pi.add_int(var_name.clone())
}


pub fn assign_symvar_value(&mut self, mut stmt_rs: String, stmt_ls: String) {
    let mut i = 0;
    while i < self.sigma.len() {
        if stmt_rs.contains(&self.sigma[i].name) {
            let s = format!("({})", self.sigma[i].var0);
            stmt_rs = stmt_rs.replace(&self.sigma[i].name, &s);
        }
        i = i + 1;
    }
    let mut j = 0;
    let mut found = false;
    while j < self.sigma.len() {
        if stmt_ls.contains(&self.sigma[j].name) {
            found = true;
            self.sigma[j].prev = self.sigma[j].var0.clone();
            self.sigma[j].var0 = eval(stmt_rs.clone());
        }
        j = j + 1;
    }
    if !found {
        panic!("assignment to uninitialized variable attempted");
    }
}
}

#[cfg(test)]
mod tests {
    use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
    use crate::symex::*;
    use equation_solver::*;
    static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

    #[test]
    fn demo_engine() {
        let init_engine = || -> Result<(), Box<dyn std::error::Error>> {
            let mut e = SymExEngine {
                pi: SymSolver::new(),
                sigma: Vec::new(),
                path: 1,
            };
            //demonstration of updating sigma using predetermined variables and assignments
            e.new_variable("x".to_string(), "i32".to_string());
            e.new_variable_assign("y".to_string(), "u64".to_string(), "5 + 6".to_string());
            e.assign_symvar_value("x + 4".to_string(), "x".to_string());
            e.assign_symvar_value("y * x".to_string(), "y".to_string());

            println!("{}", e.to_string());

            Ok(())
        };
    
        if let Err(_err) = init_engine() {
            println!("Failed to initialize symbolic execution engine.");
        }
    }
}