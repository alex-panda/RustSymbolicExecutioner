mod parser;
mod compiler;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File name expected");
        return;
    }
    println!("{}", &args[1]);

    let valid = compiler::compile_input(&args[1]);
    if valid {
        println!("Hello World!");
    }

    else {
        println!("Could not compile");
    }
}