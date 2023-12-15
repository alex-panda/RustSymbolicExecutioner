#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i8, i8)| {
    if let z = data {
        let mut x:i32 = data.0.into();
        let mut y:i32 = data.1.into();
        y = 2;
        if x < 4 {
            y = 4;
        }
    
        else if x > 4 {
            y = 2;
        }
    
        else {
            y = 0;
            panic!("found a value to reach this condition");
        }
    }
});
