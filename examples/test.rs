fn main() {
    let mut x = 5;
    let mut y = 18;

    s_algebra(x, y);
}

fn s_algebra(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    let mut w = (x*4) + y;
    //symex - what are the possible values
    return 0;
}

fn b_algebra(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    let mut w = x / y;
    //symex - division by zero?
    return 0;
}

fn b_ifStmt(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    if x <= 4 {
        x = 4;
    }

    else if x > 4 {
        x = 2;
    }

    else {
        y = 0;
        //symex - is this reachable?
    }
    //symex - what can the value of y be
    return 0;
}
fn s_ifStmt(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    if x < 4 {
        x = 4;
    }

    else if x > 4 {
        x = 2;
    }

    else {
        y = 0;
        //symex - is this reachable?
    }
    return 0;
}

fn b2_ifStmt(mut x:i32, mut y:i32) -> u8 {
    if x < 5 {
        if x >= 5 {
            y = x;
        }
        x = y * 2;
        //symex
    }
    return 0;
}

fn s2_ifStmt(mut x:i32, mut y:i32) -> u8 {
    if x < 5 {
        if x > 5 {
            y = x;
        }
        x = y * 2;
        //symex
    }
    return 0;
}

fn s_loop(n: i64)
{
    let mut i: i64 = 0;
    let mut j: i64 = 1;
    while i < n {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
}

fn b_loop(n: i64)
{
    let mut i: i64 = 0;
    let mut j: i64 = 1;
    while i <= n {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
}