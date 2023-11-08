#![no_main]

use areion::digest::Digest;
use areion::Areion512Md;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    Areion512Md::new().chain_update(data).finalize();
});
