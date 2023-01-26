#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    lthash_rs::LtHash16::<sha3::Shake256>::try_from_bytes(data).ok();
});
