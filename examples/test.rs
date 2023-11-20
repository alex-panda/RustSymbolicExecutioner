fn main() {
    let mut x = 5;
    let mut y = 18;

    a(x, y);
}

fn a(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    //symex
    return 0;
}