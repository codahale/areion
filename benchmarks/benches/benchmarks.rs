use areion::digest::Digest;
use areion::{Areion256Sponge, AreionHaifa512};
use divan::counter::BytesCount;
use divan::Bencher;
use sha2::{Sha256, Sha512};

#[divan::bench(counters = [BytesCount::new(32usize)])]
fn areion256(b: Bencher) {
    b.with_inputs(|| (areion::load(&[0u8; 16]), areion::load(&[0u8; 16])))
        .bench_values(|(x0, x1)| areion::areion256(x0, x1))
}

#[divan::bench(counters = [BytesCount::new(64usize)])]
fn areion512(b: Bencher) {
    b.with_inputs(|| {
        (
            areion::load(&[0u8; 16]),
            areion::load(&[0u8; 16]),
            areion::load(&[0u8; 16]),
            areion::load(&[0u8; 16]),
        )
    })
    .bench_values(|(x0, x1, x2, x3)| areion::areion512(x0, x1, x2, x3))
}

#[divan::bench(counters = [BytesCount::new(32usize)])]
fn areion256_dm(b: Bencher) {
    b.with_inputs(|| (areion::load(&[0u8; 16]), areion::load(&[0u8; 16])))
        .bench_values(|(x0, x1)| areion::areion256_dm(x0, x1))
}

#[divan::bench(counters = [BytesCount::new(64usize)])]
fn areion512_dm(b: Bencher) {
    b.with_inputs(|| {
        (
            areion::load(&[0u8; 16]),
            areion::load(&[0u8; 16]),
            areion::load(&[0u8; 16]),
            areion::load(&[0u8; 16]),
        )
    })
    .bench_values(|(x0, x1, x2, x3)| areion::areion512_dm(x0, x1, x2, x3))
}

const LENS: &[usize] = &[16, 256, 1024, 16 * 1024, 1024 * 1024];

#[divan::bench(args = LENS)]
fn areion512_md(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| areion::Areion512Md::default().chain_update(block).finalize());
}

#[divan::bench(args = LENS)]
fn areion512_mmo(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| areion::Areion512Mmo::default().chain_update(block).finalize());
}

#[divan::bench(args = LENS)]
fn areion256_512_sponge(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| Areion256Sponge::new().chain_update(block).finalize());
}

#[divan::bench(args = LENS)]
fn areion512_haifa(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| AreionHaifa512::new().chain_update(block).finalize());
}

#[divan::bench(args = LENS)]
fn sha256(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| Sha256::new().chain_update(block).finalize());
}

#[divan::bench(args = LENS)]
fn blake3(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| blake3::hash(block));
}

#[divan::bench(args = LENS)]
fn sha512(bencher: divan::Bencher, len: usize) {
    bencher
        .with_inputs(|| vec![0u8; len])
        .counter(BytesCount::new(len))
        .bench_refs(|block| Sha512::new().chain_update(block).finalize());
}

fn main() {
    divan::main();
}
