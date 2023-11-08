use hex_literal::hex;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "aarch64")]
use crate::aarch64::{self as internal, *};

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
use crate::x86_64::{self as internal, *};

pub use internal::zero;

mod haifa;
#[cfg(target_arch = "aarch64")]
mod md;
mod mmo;
mod sponge;

pub use crate::haifa::{AreionHaifa512, AreionHaifaVar};
#[cfg(target_arch = "aarch64")]
pub use crate::md::Areion512Md;
pub use crate::mmo::Areion512Mmo;
pub use crate::sponge::Areion256Sponge;

#[inline]
#[allow(clippy::identity_op)]
unsafe fn simpira_f<const C: u32, const B: u32>(x: AesBlock) -> AesBlock {
    let c = load_32x4(0x00 ^ C ^ B, 0x10 ^ C ^ B, 0x20 ^ C ^ B, 0x30 ^ C ^ B);
    enc(enc(x, c), zero())
}

pub fn simpira_v2_b2(mut x0: AesBlock, mut x1: AesBlock) -> (AesBlock, AesBlock) {
    unsafe {
        (x0, x1) = (xor(simpira_f::<1, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<2, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<3, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<4, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<5, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<6, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<7, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<8, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<9, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<10, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<11, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<12, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<13, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<14, 2>(x0), x1), x0);
        (x0, x1) = (xor(simpira_f::<15, 2>(x0), x1), x0);
        (x0, x1)
    }
}

static RC0: [[u8; 16]; 24] = [
    hex!("447370032e8a1913d308a385886a3f24"),
    hex!("896c4eec98fa2e08d0319f29223809a4"),
    hex!("6c0ce934cf6654be7713d038e6212845"),
    hex!("170947b5b5d5843fdd507cc9b729acc0"),
    hex!("acb5df98a60b31d11bfb7989d9d51692"),
    hex!("967e266aedafe1b8b7df1ad0db72fd2f"),
    hex!("f76c91b34799a124997f2cf145907cba"),
    hex!("90e67415870d923666c1ef58282e1f80"),
    hex!("58b68e728f74950d7e3d93f4a3fe58a4"),
    hex!("b5595ac21da4547bee4a158258cd8b71"),
    hex!("f085602823b0d1c51360f22a39d5309c"),
    hex!("0e183a60b0dc798eef38dbb8187941ca"),
    hex!("274b31bdc17715d73e8a1eb08b0e9e6c"),
    hex!("94ab55aaf32555e6605c6055da2faf78"),
    hex!("b610ab2a6a39ca554014e86362984857"),
    hex!("93e9727caf8654a1cee84111345cccb4"),
    hex!("f63118745dc5a92b2abc6f631114eeb3"),
    hex!("5ccf246c33bad6af1e93879b163e5cce"),
    hex!("afb94b6b98488f3b778695288153327a"),
    hex!("91a921fbcc09d861932128661be8bfc4"),
    hex!("b17585e95d5d84ef3280ec5d60ac7c48"),
    hex!("c5ac96d3813e8923881b65eb022326dc"),
    hex!("400042482a48b4e09223443f38ffd6f6"),
    hex!("9a6ce9f64268c6215e9b1f9e4af0c869"),
];

#[inline]
fn round_256<const R: usize>(x0: AesBlock, x1: AesBlock) -> (AesBlock, AesBlock) {
    let rc0 = load(&RC0[R]);
    let rc1 = zero();
    let (x1, x0) = (enc(enc(x0, rc0), x1), enc_last(x0, rc1));
    (x0, x1)
}

pub fn areion256(x0: AesBlock, x1: AesBlock) -> (AesBlock, AesBlock) {
    let (x0, x1) = round_256::<0>(x0, x1);
    let (x1, x0) = round_256::<1>(x1, x0);
    let (x0, x1) = round_256::<2>(x0, x1);
    let (x1, x0) = round_256::<3>(x1, x0);
    let (x0, x1) = round_256::<4>(x0, x1);
    let (x1, x0) = round_256::<5>(x1, x0);
    let (x0, x1) = round_256::<6>(x0, x1);
    let (x1, x0) = round_256::<7>(x1, x0);
    let (x0, x1) = round_256::<8>(x0, x1);
    let (x1, x0) = round_256::<9>(x1, x0);
    (x0, x1)
}

#[inline]
fn inv_round_256<const R: usize>(x0: AesBlock, x1: AesBlock) -> (AesBlock, AesBlock) {
    let rc0 = load(&RC0[R]);
    let rc1 = zero();
    let x0 = dec_last(x0, rc1);
    let x1 = enc(enc(x0, rc0), x1);
    (x0, x1)
}

pub fn inv_areion256(x0: AesBlock, x1: AesBlock) -> (AesBlock, AesBlock) {
    let (x1, x0) = inv_round_256::<9>(x1, x0);
    let (x0, x1) = inv_round_256::<8>(x0, x1);
    let (x1, x0) = inv_round_256::<7>(x1, x0);
    let (x0, x1) = inv_round_256::<6>(x0, x1);
    let (x1, x0) = inv_round_256::<5>(x1, x0);
    let (x0, x1) = inv_round_256::<4>(x0, x1);
    let (x1, x0) = inv_round_256::<3>(x1, x0);
    let (x0, x1) = inv_round_256::<2>(x0, x1);
    let (x1, x0) = inv_round_256::<1>(x1, x0);
    let (x0, x1) = inv_round_256::<0>(x0, x1);
    (x0, x1)
}

#[inline]
fn round_512<const R: usize>(
    x0: AesBlock,
    x1: AesBlock,
    x2: AesBlock,
    x3: AesBlock,
) -> (AesBlock, AesBlock, AesBlock, AesBlock) {
    let rc0 = load(&RC0[R]);
    let rc1 = zero();
    let x1 = enc(x0, x1);
    let x3 = enc(x2, x3);
    let x0 = enc_last(x0, rc1);
    let x2 = enc(enc_last(x2, rc0), rc1);
    (x0, x1, x2, x3)
}

#[inline]
pub fn areion512(
    x0: AesBlock,
    x1: AesBlock,
    x2: AesBlock,
    x3: AesBlock,
) -> (AesBlock, AesBlock, AesBlock, AesBlock) {
    let (x0, x1, x2, x3) = round_512::<0>(x0, x1, x2, x3);
    let (x1, x2, x3, x0) = round_512::<1>(x1, x2, x3, x0);
    let (x2, x3, x0, x1) = round_512::<2>(x2, x3, x0, x1);
    let (x3, x0, x1, x2) = round_512::<3>(x3, x0, x1, x2);
    let (x0, x1, x2, x3) = round_512::<4>(x0, x1, x2, x3);
    let (x1, x2, x3, x0) = round_512::<5>(x1, x2, x3, x0);
    let (x2, x3, x0, x1) = round_512::<6>(x2, x3, x0, x1);
    let (x3, x0, x1, x2) = round_512::<7>(x3, x0, x1, x2);
    let (x0, x1, x2, x3) = round_512::<8>(x0, x1, x2, x3);
    let (x1, x2, x3, x0) = round_512::<9>(x1, x2, x3, x0);
    let (x2, x3, x0, x1) = round_512::<10>(x2, x3, x0, x1);
    let (x3, x0, x1, x2) = round_512::<11>(x3, x0, x1, x2);
    let (x0, x1, x2, x3) = round_512::<12>(x0, x1, x2, x3);
    let (x1, x2, x3, x0) = round_512::<13>(x1, x2, x3, x0);
    let (x2, x3, x0, x1) = round_512::<14>(x2, x3, x0, x1);
    (x3, x0, x1, x2)
}

#[inline]
fn inv_round_512<const R: usize>(
    x0: AesBlock,
    x1: AesBlock,
    x2: AesBlock,
    x3: AesBlock,
) -> (AesBlock, AesBlock, AesBlock, AesBlock) {
    let rc0 = load(&RC0[R]);
    let rc1 = zero();
    let x0 = dec_last(x0, rc1);
    let x2 = dec_last(dec_last(inv_mix(x2), rc0), rc1);
    let x1 = enc(x0, x1);
    let x3 = enc(x2, x3);
    (x0, x1, x2, x3)
}

pub fn inv_areion512(
    x0: AesBlock,
    x1: AesBlock,
    x2: AesBlock,
    x3: AesBlock,
) -> (AesBlock, AesBlock, AesBlock, AesBlock) {
    let (x2, x3, x0, x1) = (x3, x0, x1, x2);
    let (x2, x3, x0, x1) = inv_round_512::<14>(x2, x3, x0, x1);
    let (x1, x2, x3, x0) = inv_round_512::<13>(x1, x2, x3, x0);
    let (x0, x1, x2, x3) = inv_round_512::<12>(x0, x1, x2, x3);
    let (x3, x0, x1, x2) = inv_round_512::<11>(x3, x0, x1, x2);
    let (x2, x3, x0, x1) = inv_round_512::<10>(x2, x3, x0, x1);
    let (x1, x2, x3, x0) = inv_round_512::<9>(x1, x2, x3, x0);
    let (x0, x1, x2, x3) = inv_round_512::<8>(x0, x1, x2, x3);
    let (x3, x0, x1, x2) = inv_round_512::<7>(x3, x0, x1, x2);
    let (x2, x3, x0, x1) = inv_round_512::<6>(x2, x3, x0, x1);
    let (x1, x2, x3, x0) = inv_round_512::<5>(x1, x2, x3, x0);
    let (x0, x1, x2, x3) = inv_round_512::<4>(x0, x1, x2, x3);
    let (x3, x0, x1, x2) = inv_round_512::<3>(x3, x0, x1, x2);
    let (x2, x3, x0, x1) = inv_round_512::<2>(x2, x3, x0, x1);
    let (x1, x2, x3, x0) = inv_round_512::<1>(x1, x2, x3, x0);
    let (x0, x1, x2, x3) = inv_round_512::<0>(x0, x1, x2, x3);
    (x0, x1, x2, x3)
}

pub fn areion256_dm(x0: AesBlock, x1: AesBlock) -> (AesBlock, AesBlock) {
    let (x0_p, x1_p) = areion256(x0, x1);
    (xor(x0_p, x0), xor(x1_p, x1))
}

pub fn areion512_dm(
    x0: AesBlock,
    x1: AesBlock,
    x2: AesBlock,
    x3: AesBlock,
) -> (AesBlock, AesBlock) {
    let (x0_p, x1_p, x2_p, x3_p) = areion512(x0, x1, x2, x3);
    let (x0_p, x1_p, x2_p, x3_p) = (xor(x0_p, x0), xor(x1_p, x1), xor(x2_p, x2), xor(x3_p, x3));

    let mut x = [0u32; 16];
    store_u32(&mut x[..4], x0_p);
    store_u32(&mut x[4..8], x1_p);
    store_u32(&mut x[8..12], x2_p);
    store_u32(&mut x[12..], x3_p);

    (
        load_32x4(x[2], x[3], x[6], x[7]),
        load_32x4(x[8], x[9], x[12], x[13]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "aarch64")]
    use digest::Digest;
    use expect_test::expect;
    use hex_literal::hex;

    #[test]
    fn perm256_inversion() {
        let x0 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x1 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let (x0, x1) = areion256(x0, x1);
        let (x0, x1) = inv_areion256(x0, x1);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..], x1);
        expect![[r#"
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn perm512_inversion() {
        let x0 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x1 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x2 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x3 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);
        let (x0, x1, x2, x3) = inv_areion512(x0, x1, x2, x3);

        let mut x_p = [0u8; 64];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..32], x1);
        store(&mut x_p[32..48], x2);
        store(&mut x_p[48..], x3);
        expect![[r#"
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn perm256_test_vector_1() {
        let x0 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x1 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let (x0, x1) = areion256(x0, x1);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..], x1);
        expect![[r#"
                28 12 a7 24 65 b2 6e 9f ca 75 83 f6 e4 12 3a a1
                49 0e 35 e7 d5 20 3e 4b a2 e9 27 b0 48 2f 4d b8"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn perm256_test_vector_2() {
        let x0 = load(&hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
        let x1 = load(&hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
        let (x0, x1) = areion256(x0, x1);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..], x1);
        expect![[r#"
                68 84 5f 13 2e e4 61 60 66 c7 02 d9 42 a3 b2 c3
                a3 77 f6 5b 13 bb 05 c7 cd 1f b2 9c 89 af a1 85"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn perm512_test_vector_1() {
        let x0 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x1 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x2 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x3 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);

        let mut x_p = [0u8; 64];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..32], x1);
        store(&mut x_p[32..48], x2);
        store(&mut x_p[48..], x3);
        expect![[r#"
                b2 ad b0 4f a9 1f 90 15 59 36 71 22 cb 3c 96 a9
                78 cf 3e e4 b7 3c 6a 54 3f e6 dc 85 77 91 02 e7
                e3 f5 50 10 16 ce ed 1d d2 c4 8d 0b c2 12 fb 07
                ad 16 87 94 bd 96 cf f3 59 09 cd d8 e2 27 49 28"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn perm512_test_vector_2() {
        let x0 = load(&hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
        let x1 = load(&hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
        let x2 = load(&hex!("20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f"));
        let x3 = load(&hex!("30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f"));
        let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);

        let mut x_p = [0u8; 64];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..32], x1);
        store(&mut x_p[32..48], x2);
        store(&mut x_p[48..], x3);
        expect![[r#"
                b6 90 b8 82 97 ec 47 0b 07 dd a9 2b 91 95 9c ff
                13 5e 9a c5 fc 3d c9 b6 47 a4 3f 4d aa 8d a7 a4
                e0 af bd d8 e6 e2 55 c2 45 27 73 6b 29 8b d6 1d
                e4 60 ba b9 ea 79 15 c6 d6 dd be 05 fe 8d de 40"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn areion256_dm_test_vector_1() {
        let x0 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x1 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let (x0, x1) = areion256_dm(x0, x1);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..], x1);
        expect![[r#"
                28 12 a7 24 65 b2 6e 9f ca 75 83 f6 e4 12 3a a1
                49 0e 35 e7 d5 20 3e 4b a2 e9 27 b0 48 2f 4d b8"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn areion256_dm_test_vector_2() {
        let x0 = load(&hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
        let x1 = load(&hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
        let (x0, x1) = areion256_dm(x0, x1);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..], x1);
        expect![[r#"
                68 85 5d 10 2a e1 67 67 6e ce 08 d2 4e ae bc cc
                b3 66 e4 48 07 ae 13 d0 d5 06 a8 87 95 b2 bf 9a"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn areion512_dm_test_vector_1() {
        let x0 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x1 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x2 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let x3 = load(&hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
        let (x0, x1) = areion512_dm(x0, x1, x2, x3);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..32], x1);
        expect![[r#"
                59 36 71 22 cb 3c 96 a9 3f e6 dc 85 77 91 02 e7
                e3 f5 50 10 16 ce ed 1d ad 16 87 94 bd 96 cf f3"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[test]
    fn areion512_dm_test_vector_2() {
        let x0 = load(&hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
        let x1 = load(&hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
        let x2 = load(&hex!("20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f"));
        let x3 = load(&hex!("30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f"));
        let (x0, x1) = areion512_dm(x0, x1, x2, x3);

        let mut x_p = [0u8; 32];
        store(&mut x_p[..16], x0);
        store(&mut x_p[16..32], x1);
        expect![[r#"
                0f d4 a3 20 9d 98 92 f0 5f bd 25 56 b6 90 b9 bb
                c0 8e 9f fb c2 c7 73 e5 d4 51 88 8a de 4c 23 f1"#]]
        .assert_eq(&hex_fmt(&x_p));
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn areion512_md_test_vector_1() {
        let data = hex!(
            "
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
            00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"
        );

        expect![[r#"
                7f 22 34 44 5f 3a 72 00 65 93 79 42 01 53 6c 94
                09 5d ab d3 fd b5 84 67 48 d3 59 55 5c 52 e6 51"#]]
        .assert_eq(&hex_fmt(
            &Areion512Md::default().chain_update(data).finalize(),
        ));
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn areion512_md_test_vector_2() {
        let data = hex!(
            "
            00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f
            10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f
            20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f
            30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f
            40 41 42 43 44 45 46 47 48 49 4a 4b 4c 4d 4e 4f
            50 51 52 53 54 55 56 57 58 59 5a 5b 5c 5d 5e 5f
            60 61 62 63 64 65 66 67 68 69 6a 6b 6c 6d 6e 6f
            70 71 72 73 74 75 76 77 78 79 7a 7b 7c 7d 7e 7f"
        );

        expect![[r#"
                3e 4d 31 0f be 21 d0 7b b9 00 46 88 a1 50 36 b7
                ab d9 ae 2f e9 e6 0c 9a ca 2a cc 36 98 5e 60 0b"#]]
        .assert_eq(&hex_fmt(
            &Areion512Md::default().chain_update(data).finalize(),
        ));
    }

    fn hex_fmt(b: &[u8]) -> String {
        b.iter()
            .map(|v| format!("{:02x}", v))
            .collect::<Vec<String>>()
            .chunks(16)
            .map(|v| v.join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
