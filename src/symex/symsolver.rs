use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};

static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

pub struct SymSolver {
    s: Solver<Z3Binary>,
    var: Vec<Const<Int>>,
    pi_str: String,
}

impl SymSolver {
    pub fn new() -> Self {
        SymSolver {
            s: Solver::new(Z3Binary::new(PATH_TO_SOLVER).expect("Path to Z3 not found")).expect("Could not create Z3 Solver"),
            var: Vec::new(),
            pi_str: "true".to_string(),
        }
    }

    pub fn copy_solver(&self) -> SymSolver {
        let solver = &self.s;
        SymSolver {
            s: Solver::new(Z3Binary::new(PATH_TO_SOLVER).expect("Path to Z3 not found")).expect("Could not create Z3 Solver"),
            var: self.var.clone(),
            pi_str: self.pi_str.clone()
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}", &self.pi_str)
    }

    pub fn add_int(&mut self, v: String) {
        self.var.push(Int::from_name(v));
    }

    pub fn find_int(&self, v:&str) -> Const<Int> {
        let find = "|".to_owned() + v + "|";
        let mut i = 0;
        while i < self.var.len() {
            if self.var[i].name() == find {
                return self.var[i];
            }
            i = i + 1;
        }
        panic!("No Int found matching {}", v);
    }

    pub fn add_assertion_to_pi_str(&mut self, assert: &String) {
        self.pi_str = self.pi_str.clone() + assert;
    }
}

#[cfg(test)]
mod tests {
    use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};
    use crate::symex::*;
    static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

   
    #[test]
    fn solver_example() -> Result<(), Box<dyn std::error::Error>> {
        let mut solver = Solver::new(Z3Binary::new(PATH_TO_SOLVER)?)?;
        let arg_vec = vec!["x", "y", "q", "test"];
        let mut int_vec = Vec::new();
        let mut i = 0;
    
        while i < arg_vec.len() {
            int_vec.push(Int::from_name(arg_vec[i]));
            solver.assert(int_vec[i]._neq((i64::try_from(i).unwrap() + 1)* 6))?;
            //solver.assert("(= (+ (* 5 6) (* 5 6)) 7)")?;
            i = i + 1;
        }
    
        match solver.check_sat_with_model()? {
            SatResultWithModel::Sat(model) => {
                println!("Model : {model}");
                let mut j = 0;
                while j < arg_vec.len() {
                    println!("{} = {}", int_vec[j].name(), model.eval(int_vec[j]).unwrap());
                    j = j + 1;
                }
            }
            SatResultWithModel::Unsat => println!("No valid solutions found!"),
            SatResultWithModel::Unknown => println!("Satisfaction remains unknown..."),
        }
    
        Ok(())
    
    }
}