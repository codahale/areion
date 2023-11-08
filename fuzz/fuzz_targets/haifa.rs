#![no_main]

use areion::digest::Digest;
use areion::AreionHaifa512;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    AreionHaifa512::new().chain_update(data).finalize();
});
