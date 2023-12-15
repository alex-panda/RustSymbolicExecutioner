#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i8, i8)| {
    if let z = data {
        let mut x:i32 = data.0.into();
        let mut y:i32 = data.1.into();
        x = y * 2;
        if x == 6 {
            y = y + 3;
            if y > 2 {
                y = y + 2;
                panic!("found the path we want");
            }
        } else {
            y = y + 4;
        }
    
    }
});
