#![no_main]

use areion::digest::Digest;
use areion::Areion512Mmo;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    Areion512Mmo::new().chain_update(data).finalize();
});
