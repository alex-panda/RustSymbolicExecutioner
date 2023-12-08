fn main() { }

fn b_algebra(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    let w = x / y;
    //symex - division by zero?
    return w;
}