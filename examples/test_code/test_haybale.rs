fn main() {
    let mut x = 5;
    let mut y = 18;

    s_algebra(x, y);
}

fn s_algebra(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    let mut w = (x*4) + y;
    //symex - what are the possible values?
    return w;
}

fn b_algebra(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    let mut w = x / y;
    //symex - division by zero?
    return w;
}

fn b_ifStmt(mut x:i32, mut y:i32) -> i32 {
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
    return y;
}
fn s_ifStmt(mut x:i32, mut y:i32) -> i32 {
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
    return y;
}

fn b2_ifStmt(mut x:i32, mut y:i32) -> i32 {
    if x < 5 {
        if x >= 5 {
            y = x;
        }
        y = y + 1;
        //symex - what values can y have?
    }
    return y;
}

fn s2_ifStmt(mut x:i32, mut y:i32) -> i32 {
    if x < 5 {
        if x > 5 {
            y = x;
        }
        x = y * 2;
        //symex
    }
    return x;
}

fn s_loop(n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 1;
    while i < n {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
	return i;
}

fn b_loop(n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 1;
    while i <= n {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
	return i;
}

fn b_infLoop(n: i32) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 1;
    while i < n {
        j = j * 2;
    }
	return i;
}