static PATH_TO_SOLVER:&str = "z3\\bin\\z3.exe";
use rsmt2::*;
use std::rc::Rc;
pub struct SymSolver {
    pub s: Solver<()>,
    pub pi_str: String,
    int_str: String,
    assert_str: String,
}

pub fn format_assertion(assert: &String) -> String {
    return "(= 5 5)".to_string();
}

impl SymSolver {
    pub fn new() -> Self {
        
        SymSolver {
            s: SmtConf::z3(PATH_TO_SOLVER).spawn(()).unwrap(),
            pi_str: "true".to_string(),
            int_str: "".to_string(),
            assert_str: "".to_string(),
        }
    }

    pub fn copy_solver(&self) -> SymSolver {
        let mut s = SmtConf::z3(PATH_TO_SOLVER).spawn(()).unwrap();
        let mut ints = self.int_str.split("#").collect::<Vec<&str>>();
        if ints.len() > 0 {
            ints.drain(0..1);
            let mut i = 0;
            while i < ints.len() {
                s.declare_const(ints[i].clone(), "Int");
                i = i + 1;
            }
        }

        let mut asserts = self.assert_str.split("#").collect::<Vec<&str>>();
        if asserts.len() > 0 {
            asserts.drain(0..1);
            let mut j = 0;
            while j < asserts.len() {
                s.assert(asserts[j].clone().to_string());
                j = j + 1;
            }
        }
        SymSolver {
            s: s,
            pi_str: self.pi_str.clone(),
            int_str: self.int_str.clone(),
            assert_str: self.assert_str.clone()
        }
    }
    pub fn add_assertion_to_solver(&mut self, assert: &String) {
        let fmt_assert = format_assertion(assert);
        self.s.assert(fmt_assert.clone()).unwrap();
        self.assert_str = format!("{}#{}", self.assert_str, fmt_assert.clone());
    }
    pub fn load_solver(&self) -> Solver<()> {
        let mut s = SmtConf::z3(PATH_TO_SOLVER).spawn(()).unwrap();
        let ints = self.int_str.split("#");
        for i in ints {
            println!("{}", i);
            //s.declare_const(i.clone(), "Int");
        }

        let asserts = self.assert_str.split("#");
        for a in asserts {
            println!("{}", a);
            s.assert(a.clone().to_string());
        }
        return s;
    }

    pub fn to_string(&self) -> String {
        format!("{}", &self.pi_str)
    }

    pub fn add_int(&mut self, v: String) {
        self.s.declare_const(v.clone(), "Int");
        self.int_str = format!("{}#{}", self.int_str, v.clone());
    }

    pub fn add_assertion_to_pi_str(&mut self, assert: &String) {
        self.pi_str = self.pi_str.clone() + assert;
    }
}

#[cfg(test)]
mod tests {

}