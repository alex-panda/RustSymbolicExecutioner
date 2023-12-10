use equation_solver::*;
use crate::symex::{SymVar, SymSolver};

pub struct SymExEngine {
    pub fn_name: String,
    pub pi: SymSolver,
    pub sigma: Vec<SymVar>,
    pub path: u32,
    pub reached_symex: bool,
}

pub fn eval(stmt_rs: String) -> String {
    let stmt_clean = stmt_rs.replace(";", "");
    //println!("eval {}", stmt_clean);
    let n = Equation::new(stmt_clean.clone());

    let eq = n.unwrap();

    let wrap_result = eq.evaluate();
    
    let eval = match wrap_result {
        Ok(_) => wrap_result.unwrap().to_string(), // f64
        Err(_) => stmt_clean.clone(), // EquationError
    };

    return eval;
}



impl SymExEngine {
    pub fn to_string(&self) -> String {
        let mut i = 0;
        let mut s: String = "".to_string();
        while i < self.sigma.len() {
            s = s + "\t" + &self.sigma[i].to_string() + "\n";
            i = i + 1;
        }
        format!("function: {}\npath id: {}\nsatisfiable: {}\npi: {}\nsigma: {}", &self.fn_name, &self.path, &self.pi.satisfiable.to_string(), &self.pi.to_string(), s)
    }

    //creates symvar from function header
//ie (mut var_name: var_type)
    pub fn new_variable(&mut self, var_name: String, var_type: String) {
        let v = SymVar::new(var_name.clone(), var_type.clone());
        //println!("created {} of type {}", var_name.clone(), var_type.clone());
        self.sigma.push(v);
        self.pi.add_int(var_name.clone());
    }

    //creates symvar from initialization
    //ie let var_name: var_type = assign;
    pub fn new_variable_assign(&mut self, var_name: String, var_type: String, assign: String, l_assign: String) {
        let a = assign.replace(";", "");
        let stmt = self.display_as_var0(a);
        let v = SymVar::new_assign(var_name.clone(), var_type.clone(), eval(stmt.clone()), l_assign);
        //println!("created {} of type {} with value {}", var_name.clone(), var_type.clone(), assign.clone());
        self.sigma.push(v);
        self.pi.add_int(var_name.clone())
    }

    pub fn display_as_var0(&mut self, st: String) -> String {
        let mut stmt = st.replace(";", "");
        let mut i = 0;
        while i < self.sigma.len() {
            let f = &self.sigma[i].name.eq(&self.sigma[i].var0);
            if stmt.contains(&self.sigma[i].name) && !f {
                let s = format!("({})", self.sigma[i].var0);
                stmt = stmt.replace(&self.sigma[i].name, &s);
            }
            else if *f {
                let s = format!("{}", self.sigma[i].var0);
                //println!("{}", stmt);
                stmt = stmt.replace(&self.sigma[i].name, &s);
            }
            i = i + 1;
        }
        
        return stmt;
    }

    pub fn lisp_as_var0(&mut self, mut stmt: String) -> String {
        let mut i = 0;
        while i < self.sigma.len() {
            let f = &self.sigma[i].name.eq(&self.sigma[i].var0);
            if stmt.contains(&self.sigma[i].name) && !f {
                let s = format!("{}", self.sigma[i].lisp_var0);
                stmt = stmt.replace(&self.sigma[i].name, &s);
            }
            else if *f {
                let s = format!("{}", self.sigma[i].lisp_var0);
                
                stmt = stmt.replace(&self.sigma[i].name, &s);
            }
            i = i + 1;
        }
       // println!("{}", stmt);
        return stmt;
    }

    pub fn assign_symvar_value(&mut self, mut stmt_rs: String, stmt_ls: String, mut lisp_rs: String) {
        //println!("{} = {}", stmt_ls.clone(), stmt_rs.clone());
        stmt_rs = self.display_as_var0(stmt_rs);
        lisp_rs = self.lisp_as_var0(lisp_rs);
        let mut j = 0;
        let mut found = false;
        while j < self.sigma.len() {
            if stmt_ls.contains(&self.sigma[j].name) {
                found = true;
                self.sigma[j].prev = self.sigma[j].var0.clone();
                self.sigma[j].var0 = eval(stmt_rs.clone());
                self.sigma[j].lisp_var0 = lisp_rs.clone();
                self.sort_symvar(j);
            }
            j = j + 1;
        }
        if !found {
            panic!("assignment to uninitialized variable attempted");
        }
    }

    pub fn new_assertion(&mut self, a: String, lisp: String) {
        let assert = a.replace(";", "");
        let var0_assert = self.display_as_var0(assert.clone());
        let lisp_assert =  self.lisp_as_var0(lisp.clone());
        let and_assert = " && ".to_owned() + &var0_assert;
        self.pi.add_assertion_to_pi_str(&and_assert);
        self.pi.add_assertion_to_solver(&lisp_assert);
    }


    pub fn sort_symvar(&mut self, i: usize) {
        let mut temp= i;

        while temp > 0 {
            self.sigma.swap(temp, temp - 1);
            temp = temp - 1;
        }

        //println!("{}", self.to_string());

    }
}

//#[cfg(test)]
//mod tests {
//    use crate::symex::*;
//    use equation_solver::*;
//    static PATH_TO_SOLVER:&str = "z3\\bin\\z3";
//
//
//}