#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i8, i8)| {
    if let z = data {
        let mut x:i32 = data.0.into();
        let mut y:i32 = data.1.into();
        if x < 5 {
            if x > 5 {
                y = x;
            }
            x = y * 2;
            //symex        }
    
        }
        if x - (y*2) == 0 {
            panic!("found path");
        }
    }
});
