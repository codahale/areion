#![no_main]

use areion::digest::Digest;
use areion::Areion256Sponge;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    Areion256Sponge::new().chain_update(data).finalize();
});
