use areion::{areion256_dm, areion512_dm, perm256, perm512};
use std::arch::aarch64::vmovq_n_u8;

use criterion::{criterion_group, criterion_main, Criterion};

fn perm(c: &mut Criterion) {
    unsafe {
        let x0 = vmovq_n_u8(0);
        let x1 = vmovq_n_u8(0);
        let x2 = vmovq_n_u8(0);
        let x3 = vmovq_n_u8(0);
        c.bench_function("perm256", |b| b.iter(|| perm256(x0, x1)));
        c.bench_function("perm512", |b| b.iter(|| perm512(x0, x1, x2, x3)));
    }
}

fn dm(c: &mut Criterion) {
    unsafe {
        let x0 = vmovq_n_u8(0);
        let x1 = vmovq_n_u8(0);
        let x2 = vmovq_n_u8(0);
        let x3 = vmovq_n_u8(0);
        c.bench_function("areion256-dm", |b| b.iter(|| areion256_dm(x0, x1)));
        c.bench_function("areion512-dm", |b| b.iter(|| areion512_dm(x0, x1, x2, x3)));
    }
}

criterion_group!(all, perm, dm);
criterion_main!(all);
