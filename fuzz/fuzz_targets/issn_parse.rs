#![no_main]

use issn::Issn;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = Issn::new(data);
});
