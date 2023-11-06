use areion::Areion256Sponge;
use divan::counter::BytesCount;
use divan::Bencher;
use sha2::digest::Digest;
use sha2::{Sha256, Sha512};

#[divan::bench(counters = [BytesCount::new(32usize)])]
fn areion256(b: Bencher) {
    b.with_inputs(|| (areion::zero(), areion::zero()))
        .bench_values(|(x0, x1)| areion::areion256(x0, x1))
}

#[divan::bench(counters = [BytesCount::new(64usize)])]
fn areion512(b: Bencher) {
    b.with_inputs(|| {
        (
            areion::zero(),
            areion::zero(),
            areion::zero(),
            areion::zero(),
        )
    })
    .bench_values(|(x0, x1, x2, x3)| areion::areion512(x0, x1, x2, x3))
}

#[cfg(target_arch = "aarch64")]
#[divan::bench(counters = [BytesCount::new(32usize)])]
fn areion256_dm(b: Bencher) {
    b.with_inputs(|| (areion::zero(), areion::zero()))
        .bench_values(|(x0, x1)| areion::areion256_dm(x0, x1))
}

#[cfg(target_arch = "aarch64")]
#[divan::bench(counters = [BytesCount::new(64usize)])]
fn areion512_dm(b: Bencher) {
    b.with_inputs(|| {
        (
            areion::zero(),
            areion::zero(),
            areion::zero(),
            areion::zero(),
        )
    })
    .bench_values(|(x0, x1, x2, x3)| areion::areion512_dm(x0, x1, x2, x3))
}

#[divan::bench(counters = [BytesCount::new(32usize)])]
fn simpira_v2_b2(b: Bencher) {
    b.with_inputs(|| (areion::zero(), areion::zero()))
        .bench_values(|(x0, x1)| areion::simpira_v2_b2(x0, x1))
}

const LENS: &[usize] = &[16, 256, 1024, 16 * 1024, 1024 * 1024];

#[cfg(target_arch = "aarch64")]
#[divan::bench(consts = LENS)]
fn areion512_md<const LEN: usize>(bencher: divan::Bencher) {
    use digest::Digest;
    bencher
        .with_inputs(|| vec![0u8; LEN])
        .counter(BytesCount::new(LEN))
        .bench_refs(|block| {
            areion::Areion512Md::default()
                .chain_update(block)
                .finalize()
        });
}

#[divan::bench(consts = LENS)]
fn areion512_mmo<const LEN: usize>(bencher: divan::Bencher) {
    use digest::Digest;
    bencher
        .with_inputs(|| vec![0u8; LEN])
        .counter(BytesCount::new(LEN))
        .bench_refs(|block| {
            areion::Areion512Mmo::default()
                .chain_update(block)
                .finalize()
        });
}

#[divan::bench(consts = LENS)]
fn areion256sponge<const LEN: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| vec![0u8; LEN])
        .counter(BytesCount::new(LEN))
        .bench_refs(|block| Areion256Sponge::new().chain_update(block).finalize());
}

#[divan::bench(consts = LENS)]
fn sha256<const LEN: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| vec![0u8; LEN])
        .counter(BytesCount::new(LEN))
        .bench_refs(|block| Sha256::new().chain_update(block).finalize());
}

#[divan::bench(consts = LENS)]
fn sha512<const LEN: usize>(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| vec![0u8; LEN])
        .counter(BytesCount::new(LEN))
        .bench_refs(|block| Sha512::new().chain_update(block).finalize());
}

fn main() {
    divan::main();
}
