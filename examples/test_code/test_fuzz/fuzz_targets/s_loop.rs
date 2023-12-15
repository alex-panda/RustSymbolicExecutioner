#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: i8| {
    if let n = data {
        let mut i:i32 = 0;
        let mut j:i32 = 1;
        while i < n.into() {
            j = j * 2;
            i = i + 1;
        }
        if i == n.into() {
            panic!("out of bounds");
        }
    }
});

