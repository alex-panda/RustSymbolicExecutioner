fn a(x: i32, y: i32) {
    x = 3;
    if x > 5 {
        y += 1;
    }
}


fn b(x: i32, y: i32) {
    x = y + 5;
    if x >= 5 {
        y += 1;
    }
}

fn c(x: i32, y: i32) {
    if x > 3 {
        y = y + 5;
    }

    if y <= 0 {
        x = 3;
    }
}
