#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i8, i8)| {
    if let z = data {
        let mut y:i32 = z.0.into();
        let mut x:i32 = z.1.into();
        y = y + x;
        let w = x / y;
    }
});