static PATH_TO_SOLVER:&str = "z3\\bin\\z3.exe";
use rsmt2::*;
use std::rc::Rc;
pub struct SymSolver {
    s: Solver<()>,
    pi_str: String,
}

impl SymSolver {
    pub fn new() -> Self {
        SymSolver {
            s: SmtConf::z3(PATH_TO_SOLVER).spawn(()).unwrap(),
            pi_str: "true".to_string(),
        }
    }

    pub fn copy_solver(&self) -> SymSolver {
        let solver = &self.s;
        SymSolver {
            s: SmtConf::z3(PATH_TO_SOLVER).spawn(()).unwrap(),
            pi_str: self.pi_str.clone()
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}", &self.pi_str)
    }

    pub fn add_int(&mut self, v: String) {
        self.s.declare_const(v, "Int");
    }

    pub fn add_assertion_to_pi_str(&mut self, assert: &String) {
        self.pi_str = self.pi_str.clone() + assert;
    }
}

#[cfg(test)]
mod tests {

}