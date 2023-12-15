#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i8)| {
    if let a = data {
        let mut i = 0;
        let mut j = 3;
        while i < 3 {
            i = i + 1;
            if i == 3 {
                panic!("Found a bad case");
            }
        }
    }
});