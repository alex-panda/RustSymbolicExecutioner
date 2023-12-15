fn main() {
    s_algebra(0, 0);
    b_algebra(0, 0);
    b_if_stmt(0, 0);
    b(0, 0);
    s_if_stmt(0, 0);
    s2_if_stmt(0, 0);
    b_loop(0);
    b_inf_loop(0);
    s_loop(0);
    s_nested_if_loop(0);
    b_nested_if_loop(0);
}

fn s_algebra(mut x:i32, mut y:i32) -> i32 {
    x = y + 4;
    y = 2*x; 
    let mut w = (x*4) + y;
    //symex - how does argument x affect this 
    return w;
}

fn b_algebra(mut x:i32, mut y:i32) -> i32 {
    x = x + 4;
    y = 2 * y; 
    let w = x / y;
    //symex - division by zero?
    return w;
}

fn b_if_stmt(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    if x <= 4 {
        y = 4;
    } else if x > 4 {
        y = 2;
    } else {
        y = 0;
        //symex - is this reachable?
    }
    return 0;
}
fn s_if_stmt(mut x:i32, mut y:i32) -> u8 {
    x = y + 4;
    y = 2*x; 
    if x < 4 {
        y = 4;
    }
    else if x > 4 {
        y = 2;
    }
    else {
        y = 0;
        //symex - is this reachable?
    }
    return 0;
}

fn b(mut x:i32, mut y:i32) {
    x = y * 2;
    if x == 6 {
        y = y + 3;
        if y > 2 {
            y = y + 2;
            //symex
        }
    } else {
        y = y + 4;
    }
    //symex
}


fn s2_if_stmt(mut x:i32, mut y:i32) -> u8 {
    if x < 5 {
        if x > 5 {
            y = x;
        }
        x = y * 2;
        //symex
    }
    return 0;
}

fn s_loop(n: i64) {
    let mut i: i64 = 0;
    let mut j: i64 = 1;
    while i < n {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
}

fn b_loop(n: i64) {
    let mut i: i64 = 0;
    let mut j: i64 = 1;
    while i <= n {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
}

fn b_nested_if_loop(mut a: i32) -> i32 {
    let mut i = 0;
    while i < 3 {
        i = i + 1;
        if i == 3 {
            a = 0;
            //symex
        }
    }
    i
}

fn s_nested_if_loop(mut a: i32) -> i32 {
    let mut i = 0;
    while i < 3 {
        if i == 3 {
            a = 0;
            //symex
        }
        i = i + 1;
    }
    i
}

fn b_inf_loop(n: i64) {
    let i = 0;
    let mut j = 1;
    while i < n {
        j = j * 2;
    }
    //symex
}

