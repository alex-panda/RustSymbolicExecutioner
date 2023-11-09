# RustSymbolicExecutioner
A toy symbolic executioner written for Rust in Rust.

Setup instructions (Windows):
  - Download a z3 release from here: https://github.com/Z3Prover/z3/releases
  - Extract the files
  - Move the directory containing /bin and /include to your project directory
  - Rename that directory to z3

How to run:
  - cargo run -- <path to a .rs program file>
  - The file must include fn main()
