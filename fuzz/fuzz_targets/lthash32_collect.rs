#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: Vec<&[u8]>| {
    let _: lthash_rs::LtHash32::<sha3::Shake256> = data.into_iter().collect();
});
