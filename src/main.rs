mod parser;

fn main() {
    // can do complicated setup code

    // then call the start function
    start(3, 4);
}

fn start(mut x: u32, y: u32) {
    if x > 3 {
        x -= 2;
    }

    // symex:check x == 1..3, y > 2
}

