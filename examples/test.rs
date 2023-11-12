fn main() {
    let mut x = 5;
    let mut y = 18;

    a(x, y);
}

fn a(mut x:i32, mut y:i32) {
    x = y + 4;
    y = 2*x; 
    //symex

    //expected: pi: true;
    //sigma: x = y + 4, y = 2*(y + 4)
    //path: 1

}