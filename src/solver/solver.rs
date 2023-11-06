use smtlib::{backend::Z3Binary, Int, SatResultWithModel, Solver, Sort};

static PATH_TO_SOLVER:&str = "z3\\bin\\z3";

pub fn solver_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut solver = Solver::new(Z3Binary::new(PATH_TO_SOLVER)?)?;
    let arg_vec = vec!["x", "y", "z", "test"];
    let mut int_vec = Vec::new();
    let mut i = 0;

    while i < arg_vec.len() {
        int_vec.push(Int::from_name(arg_vec[i]));
        solver.assert(int_vec[i]._neq(i64::try_from(i).unwrap() + 1))?;
        i = i + 1;
    }

    match solver.check_sat_with_model()? {
        SatResultWithModel::Sat(model) => {
            let mut j = 0;
            while j < arg_vec.len() {
                println!("{} = {}", arg_vec[j], model.eval(int_vec[j]).unwrap());
                j = j + 1;
            }
        }
        SatResultWithModel::Unsat => println!("No valid solutions found!"),
        SatResultWithModel::Unknown => println!("Satisfaction remains unknown..."),
    }

    Ok(())

}