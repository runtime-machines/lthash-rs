#![no_main]

use libfuzzer_sys::fuzz_target;
use lthash_rs::LtHash;

fuzz_target!(|data: &[u8]| {
    let mut lt = lthash_rs::LtHash16::<sha3::Shake256>::default();

    lt.insert(data);
});
