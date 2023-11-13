use smtlib::{backend::Z3Binary, Int, terms::*, SatResultWithModel, Solver, Sort};

//static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

pub fn solver_example(solver: &mut Solver<Z3Binary>) -> Result<(), Box<dyn std::error::Error>> {
    let arg_vec = vec!["x", "y", "q", "test"];
    let mut int_vec = Vec::new();
    let mut i = 0;

    while i < arg_vec.len() {
        int_vec.push(Int::from_name(arg_vec[i]));
        solver.assert(int_vec[i]._neq(i64::try_from(i).unwrap() + 1))?;
    
        i = i + 1;
    }

    find_int("q", int_vec.clone());

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

fn find_int(var:&str, vec:Vec<Const<Int>>) -> Const<Int> {
    let v = "|".to_owned() + var + "|";
    let mut i = 0;
    while i < vec.len() {
        if vec[i].name() == v {
            return vec[i];
        }
        i = i + 1;
    }
    panic!("No Int found matching {}", var);
}