#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i8, i8)| {
    if let z = data {
        let mut y:i32 = z.0.into();
        let mut x:i32 = z.1.into();
        x = y + 4;
        y = 2*x; 
        let w = (x*4) + y;
        if w == 0 {
            panic!("found path");
        }
    }
});