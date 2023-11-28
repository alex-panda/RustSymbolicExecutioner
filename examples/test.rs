fn main() {
    let y = 18;
    a(y);
}

fn a(y: i32) -> u8 {
    let x = y + 4;
    let _y = 2*x; 
    //symex
    return 0;
}