use std::process::Command;

pub fn compile_input(file: &String) -> bool {
    println!("{}2", file);
    let mut compiler = Command::new("rustc")
            .arg(file)
            .status()
            .expect("failed to execute process");
     
    return compiler.success();
}