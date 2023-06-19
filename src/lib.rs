use core::arch::aarch64::*;
use core::arch::asm;
use hex_literal::hex;

macro_rules! load {
    ($bytes:expr) => {{
        vld1q_u8($bytes.as_ptr())
    }};
}

macro_rules! store {
    ($bytes:expr, $block:expr) => {{
        vst1q_u8($bytes.as_mut_ptr(), $block)
    }};
}

macro_rules! enc {
    ($a:expr, $b:expr) => {{
        let z = vmovq_n_u8(0);
        let mut a = $a;
        // TODO replace with vaeseq_u8 and vaesmcq_u8 when that's stable
        asm!(
            "AESE {0:v}.16B, {1:v}.16B",
            "AESMC {0:v}.16B, {0:v}.16B",
            inout(vreg) a, in(vreg) z,
        );
        veorq_u8(a, $b)
    }};
}

macro_rules! enc_last {
    ($a:expr, $b:expr) => {{
        let z = vmovq_n_u8(0);
        let mut a = $a;
        // TODO replace with vaeseq_u8 when that's stable
        asm!(
            "AESE {0:v}.16B, {1:v}.16B",
            inout(vreg) a, in(vreg) z,
        );
        veorq_u8(a, $b)
    }};
}

macro_rules! dec_last {
    ($a:expr, $b:expr) => {{
        let z = vmovq_n_u8(0);
        let mut a = $a;
        // TODO replace with vaeseq_u8 when that's stable
        asm!(
            "AESD {0:v}.16B, {1:v}.16B",
            inout(vreg) a, in(vreg) z,
        );
        veorq_u8(a, $b)
    }};
}

macro_rules! inv_mix {
    ($a:expr) => {{
        let mut a = $a;
        // TODO replace with vaesimc_u8 when that's stable
        asm!(
            "AESIMC {0:v}.16B, {0:v}.16B",
            inout(vreg) a,
        );
        a
    }};
}

static RC0: [[u8; 16]; 24] = [
    hex!("886a3f24d308a3852e8a191344737003"),
    hex!("223809a4d0319f2998fa2e08896c4eec"),
    hex!("e62128457713d038cf6654be6c0ce934"),
    hex!("b729acc0dd507cc9b5d5843f170947b5"),
    hex!("d9d516921bfb7989a60b31d1acb5df98"),
    hex!("db72fd2fb7df1ad0edafe1b8967e266a"),
    hex!("45907cba997f2cf14799a124f76c91b3"),
    hex!("282e1f8066c1ef58870d923690e67415"),
    hex!("a3fe58a47e3d93f48f74950d58b68e72"),
    hex!("58cd8b71ee4a15821da4547bb5595ac2"),
    hex!("39d5309c1360f22a23b0d1c5f0856028"),
    hex!("187941caef38dbb8b0dc798e0e183a60"),
    hex!("8b0e9e6c3e8a1eb0c17715d7274b31bd"),
    hex!("da2faf78605c6055f32555e694ab55aa"),
    hex!("629848574014e8636a39ca55b610ab2a"),
    hex!("345cccb4cee84111af8654a193e9727c"),
    hex!("1114eeb32abc6f635dc5a92bf6311874"),
    hex!("163e5cce1e93879b33bad6af5ccf246c"),
    hex!("8153327a7786952898488f3bafb94b6b"),
    hex!("1be8bfc493212866cc09d86191a921fb"),
    hex!("60ac7c483280ec5d5d5d84efb17585e9"),
    hex!("022326dc881b65eb813e8923c5ac96d3"),
    hex!("38ffd6f69223443f2a48b4e040004248"),
    hex!("4af0c8695e9b1f9e4268c6219a6ce9f6"),
];

static RC1: [u8; 16] = hex!("00000000000000000000000000000000");

macro_rules! round_256 {
    ($x0:expr, $x1:expr, $i:literal) => {{
        let rc0 = load!(&RC0[$i]);
        let rc1 = load!(&RC1);
        $x1 = enc!(enc!($x0, rc0), $x1);
        $x0 = enc_last!($x0, rc1);
    }};
}

pub fn areion256(mut x0: uint8x16_t, mut x1: uint8x16_t) -> (uint8x16_t, uint8x16_t) {
    unsafe {
        round_256!(x0, x1, 0);
        round_256!(x1, x0, 1);
        round_256!(x0, x1, 2);
        round_256!(x1, x0, 3);
        round_256!(x0, x1, 4);
        round_256!(x1, x0, 5);
        round_256!(x0, x1, 6);
        round_256!(x1, x0, 7);
        round_256!(x0, x1, 8);
        round_256!(x1, x0, 9);
        (x0, x1)
    }
}

macro_rules! inv_round_256 {
    ($x0:expr, $x1:expr, $i:literal) => {{
        let rc0 = load!(&RC0[$i]);
        let rc1 = load!(&RC1);
        $x0 = dec_last!($x0, rc1);
        $x1 = enc!(enc!($x0, rc0), $x1);
    }};
}

pub fn inv_areion256(mut x0: uint8x16_t, mut x1: uint8x16_t) -> (uint8x16_t, uint8x16_t) {
    unsafe {
        inv_round_256!(x1, x0, 9);
        inv_round_256!(x0, x1, 8);
        inv_round_256!(x1, x0, 7);
        inv_round_256!(x0, x1, 6);
        inv_round_256!(x1, x0, 5);
        inv_round_256!(x0, x1, 4);
        inv_round_256!(x1, x0, 3);
        inv_round_256!(x0, x1, 2);
        inv_round_256!(x1, x0, 1);
        inv_round_256!(x0, x1, 0);
        (x0, x1)
    }
}

macro_rules! round_512 {
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $i:literal) => {{
        let rc0 = load!(&RC0[$i]);
        let rc1 = load!(&RC1);
        $x1 = enc!($x0, $x1);
        $x3 = enc!($x2, $x3);
        $x0 = enc_last!($x0, rc1);
        $x2 = enc!(enc_last!($x2, rc0), rc1);
    }};
}

pub fn areion512(
    mut x0: uint8x16_t,
    mut x1: uint8x16_t,
    mut x2: uint8x16_t,
    mut x3: uint8x16_t,
) -> (uint8x16_t, uint8x16_t, uint8x16_t, uint8x16_t) {
    unsafe {
        round_512!(x0, x1, x2, x3, 0);
        round_512!(x1, x2, x3, x0, 1);
        round_512!(x2, x3, x0, x1, 2);
        round_512!(x3, x0, x1, x2, 3);
        round_512!(x0, x1, x2, x3, 4);
        round_512!(x1, x2, x3, x0, 5);
        round_512!(x2, x3, x0, x1, 6);
        round_512!(x3, x0, x1, x2, 7);
        round_512!(x0, x1, x2, x3, 8);
        round_512!(x1, x2, x3, x0, 9);
        round_512!(x2, x3, x0, x1, 10);
        round_512!(x3, x0, x1, x2, 11);
        round_512!(x0, x1, x2, x3, 12);
        round_512!(x1, x2, x3, x0, 13);
        round_512!(x2, x3, x0, x1, 14);
        (x0, x1, x2, x3)
    }
}

macro_rules! inv_round_512 {
    ($x0:expr, $x1:expr, $x2:expr, $x3:expr, $i:literal) => {{
        let rc0 = load!(&RC0[$i]);
        let rc1 = load!(&RC1);

        $x0 = dec_last!($x0, rc1);
        $x2 = dec_last!(dec_last!(inv_mix!($x2), rc0), rc1);
        $x1 = enc!($x0, $x1);
        $x3 = enc!($x2, $x3);
    }};
}

pub fn inv_areion512(
    mut x0: uint8x16_t,
    mut x1: uint8x16_t,
    mut x2: uint8x16_t,
    mut x3: uint8x16_t,
) -> (uint8x16_t, uint8x16_t, uint8x16_t, uint8x16_t) {
    unsafe {
        inv_round_512!(x2, x3, x0, x1, 14);
        inv_round_512!(x1, x2, x3, x0, 13);
        inv_round_512!(x0, x1, x2, x3, 12);
        inv_round_512!(x3, x0, x1, x2, 11);
        inv_round_512!(x2, x3, x0, x1, 10);
        inv_round_512!(x1, x2, x3, x0, 9);
        inv_round_512!(x0, x1, x2, x3, 8);
        inv_round_512!(x3, x0, x1, x2, 7);
        inv_round_512!(x2, x3, x0, x1, 6);
        inv_round_512!(x1, x2, x3, x0, 5);
        inv_round_512!(x0, x1, x2, x3, 4);
        inv_round_512!(x3, x0, x1, x2, 3);
        inv_round_512!(x2, x3, x0, x1, 2);
        inv_round_512!(x1, x2, x3, x0, 1);
        inv_round_512!(x0, x1, x2, x3, 0);
        (x0, x1, x2, x3)
    }
}

pub fn areion256_dm(x0: uint8x16_t, x1: uint8x16_t) -> (uint8x16_t, uint8x16_t) {
    unsafe {
        let (x0_p, x1_p) = areion256(x0, x1);
        (veorq_u8(x0_p, x0), veorq_u8(x1_p, x1))
    }
}

pub fn areion512_dm(
    x0: uint8x16_t,
    x1: uint8x16_t,
    x2: uint8x16_t,
    x3: uint8x16_t,
) -> (uint8x16_t, uint8x16_t) {
    unsafe {
        let (x0_p, x1_p, x2_p, x3_p) = areion512(x0, x1, x2, x3);
        let (x0_p, x1_p, x2_p, x3_p) = (
            veorq_u8(x0_p, x0),
            veorq_u8(x1_p, x1),
            veorq_u8(x2_p, x2),
            veorq_u8(x3_p, x3),
        );

        let mut x = [0u32; 16];
        vst1q_u32(x[..4].as_mut_ptr(), vreinterpretq_u32_u8(x0_p));
        vst1q_u32(x[4..8].as_mut_ptr(), vreinterpretq_u32_u8(x1_p));
        vst1q_u32(x[8..12].as_mut_ptr(), vreinterpretq_u32_u8(x2_p));
        vst1q_u32(x[12..].as_mut_ptr(), vreinterpretq_u32_u8(x3_p));

        let x0 = [x[2], x[3], x[6], x[7]];
        let x1 = [x[8], x[9], x[12], x[13]];

        (
            vreinterpretq_u8_u32(vld1q_u32(x0.as_ptr())),
            vreinterpretq_u8_u32(vld1q_u32(x1.as_ptr())),
        )
    }
}

static H0: [u8; 16] = hex!("6a09e667bb67ae853c6ef372a54ff53a");
static H1: [u8; 16] = hex!("510e527f9b05688c1f83d9ab5be0cd19");

// FIXME tragically underspecified, does not pass test vectors
pub fn areion512_md(data: &[u8]) -> [u8; 32] {
    unsafe {
        let mut h0 = load!(H0);
        let mut h1 = load!(H1);

        let mut chunks = data.chunks_exact(32);
        for chunk in chunks.by_ref() {
            let m0 = load!(&chunk[..16]);
            let m1 = load!(&chunk[16..]);
            let (x0, x1) = areion512_dm(m0, m1, h0, h1);
            h0 = vaddq_u8(h0, x0);
            h1 = vaddq_u8(h1, x1);
        }

        let mut last = [0u8; 32];
        let n = chunks.remainder().len();
        last[..n].copy_from_slice(chunks.remainder());
        last[n] = 0x80;
        last[32 - 8..].copy_from_slice(&(data.len() as u64).to_be_bytes());

        let m0 = load!(&last[..16]);
        let m1 = load!(&last[16..]);

        let (x0, x1) = areion512_dm(m0, m1, h0, h1);
        store!(&mut last[..16], x0);
        store!(&mut last[16..], x1);

        last
    }
}

/// A single-key Even-Mansour block cipher using the Areion512 permutation.
#[allow(clippy::too_many_arguments)]
pub fn areion512_em(
    k0: uint8x16_t,
    k1: uint8x16_t,
    k2: uint8x16_t,
    k3: uint8x16_t,
    mut x0: uint8x16_t,
    mut x1: uint8x16_t,
    mut x2: uint8x16_t,
    mut x3: uint8x16_t,
) -> (uint8x16_t, uint8x16_t, uint8x16_t, uint8x16_t) {
    unsafe {
        x0 = veorq_u8(x0, k0);
        x1 = veorq_u8(x1, k1);
        x2 = veorq_u8(x2, k2);
        x3 = veorq_u8(x3, k3);
    }
    let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);
    unsafe {
        (
            veorq_u8(x0, k0),
            veorq_u8(x1, k1),
            veorq_u8(x2, k2),
            veorq_u8(x3, k3),
        )
    }
}

// A Matyas-Meyer-Oseas hash function using a single-key Even-Mansour block cipher based on the
// Areion512 permutation.
pub fn areion512_mmo(data: &[u8]) -> [u8; 64] {
    unsafe {
        let mut h0 = load!(H0);
        let mut h1 = load!(H1);
        let mut h2 = load!(H0);
        let mut h3 = load!(H1);

        let mut chunks = data.chunks_exact(64);
        for chunk in chunks.by_ref() {
            let m0 = load!(&chunk[..16]);
            let m1 = load!(&chunk[16..32]);
            let m2 = load!(&chunk[32..48]);
            let m3 = load!(&chunk[48..]);
            (h0, h1, h2, h3) = areion512_em(h0, h1, h2, h3, m0, m1, m2, m3);
        }

        let mut last = [0u8; 64];
        let n = chunks.remainder().len();
        last[..n].copy_from_slice(chunks.remainder());
        last[n] = 0x80;
        last[64 - 8..].copy_from_slice(&(data.len() as u64).to_be_bytes());

        let m0 = load!(&last[..16]);
        let m1 = load!(&last[16..32]);
        let m2 = load!(&last[32..48]);
        let m3 = load!(&last[48..]);

        (h0, h1, _, _) = areion512_em(h0, h1, h2, h3, m0, m1, m2, m3);
        store!(&mut last[..16], h0);
        store!(&mut last[16..], h1);

        last
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;
    use hex_literal::hex;

    #[test]
    fn perm256_inversion() {
        unsafe {
            let x0 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x1 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let (x0, x1) = areion256(x0, x1);
            let (x0, x1) = inv_areion256(x0, x1);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..], x1);
            expect![[r#"
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn perm512_inversion() {
        unsafe {
            let x0 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x1 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x2 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x3 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);
            let (x0, x1, x2, x3) = inv_areion512(x0, x1, x2, x3);

            let mut x_p = [0u8; 64];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..32], x1);
            store!(&mut x_p[32..48], x2);
            store!(&mut x_p[48..], x3);
            expect![[r#"
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
                00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn perm256_test_vector_1() {
        unsafe {
            let x0 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x1 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let (x0, x1) = areion256(x0, x1);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..], x1);
            expect![[r#"
                e5 a7 66 63 82 50 14 24 68 dc 9d 76 65 dd 36 9f
                8f 79 99 8b 7a a0 92 90 6f e5 1b fd eb fa c9 c1"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn perm256_test_vector_2() {
        unsafe {
            let x0 = load!(hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
            let x1 = load!(hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
            let (x0, x1) = areion256(x0, x1);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..], x1);
            expect![[r#"
                73 53 ec 51 d4 9f ad 89 ee cb 5b ef 1e a0 e4 76
                ed 6c dc dd af 34 62 0d 01 3d cc f2 a2 26 f4 57"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn perm512_test_vector_1() {
        unsafe {
            let x0 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x1 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x2 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x3 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);

            let mut x_p = [0u8; 64];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..32], x1);
            store!(&mut x_p[32..48], x2);
            store!(&mut x_p[48..], x3);
            expect![[r#"
                5f ee f7 7c bb e8 4c 79 58 08 94 59 f4 54 e9 6f
                bf 21 fa b8 35 65 cc af 91 6b cf 9c fb 63 d2 5b
                a0 26 42 fc c1 75 12 36 40 d6 a2 18 3b a6 82 b2
                0b 72 3a fc 66 68 ff f3 de c4 7c 17 61 27 b9 84"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn perm512_test_vector_2() {
        unsafe {
            let x0 = load!(hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
            let x1 = load!(hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
            let x2 = load!(hex!("20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f"));
            let x3 = load!(hex!("30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f"));
            let (x0, x1, x2, x3) = areion512(x0, x1, x2, x3);

            let mut x_p = [0u8; 64];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..32], x1);
            store!(&mut x_p[32..48], x2);
            store!(&mut x_p[48..], x3);
            expect![[r#"
                a6 09 5f e0 57 d2 83 80 ba d2 5c 28 12 b2 30 f6
                6f 07 b0 09 a3 04 98 5a f4 37 bb 60 8a 4c b8 31
                39 2a 6f 2f 48 e4 25 ef 24 11 96 21 67 2e 37 c4
                f1 9b 94 e0 e4 ea ed af b9 f4 eb 12 6a 6d 8a bb"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn areion256_dm_test_vector_1() {
        unsafe {
            let x0 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x1 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let (x0, x1) = areion256_dm(x0, x1);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..], x1);
            expect![[r#"
                e5 a7 66 63 82 50 14 24 68 dc 9d 76 65 dd 36 9f
                8f 79 99 8b 7a a0 92 90 6f e5 1b fd eb fa c9 c1"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn areion256_dm_test_vector_2() {
        unsafe {
            let x0 = load!(hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
            let x1 = load!(hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
            let (x0, x1) = areion256_dm(x0, x1);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..], x1);
            expect![[r#"
                73 52 ee 52 d0 9a ab 8e e6 c2 51 e4 12 ad ea 79
                fd 7d ce ce bb 21 74 1a 19 24 d6 e9 be 3b ea 48"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn areion512_dm_test_vector_1() {
        unsafe {
            let x0 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x1 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x2 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let x3 = load!(hex!("00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00"));
            let (x0, x1) = areion512_dm(x0, x1, x2, x3);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..32], x1);
            expect![[r#"
                58 08 94 59 f4 54 e9 6f 91 6b cf 9c fb 63 d2 5b
                a0 26 42 fc c1 75 12 36 0b 72 3a fc 66 68 ff f3"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    fn areion512_dm_test_vector_2() {
        unsafe {
            let x0 = load!(hex!("00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f"));
            let x1 = load!(hex!("10 11 12 13 14 15 16 17 18 19 1a 1b 1c 1d 1e 1f"));
            let x2 = load!(hex!("20 21 22 23 24 25 26 27 28 29 2a 2b 2c 2d 2e 2f"));
            let x3 = load!(hex!("30 31 32 33 34 35 36 37 38 39 3a 3b 3c 3d 3e 3f"));
            let (x0, x1) = areion512_dm(x0, x1, x2, x3);

            let mut x_p = [0u8; 32];
            store!(&mut x_p[..16], x0);
            store!(&mut x_p[16..32], x1);
            expect![[r#"
                b2 db 56 23 1e bf 3e f9 ec 2e a1 7b 96 51 a6 2e
                19 0b 4d 0c 6c c1 03 c8 c1 aa a6 d3 d0 df db 98"#]]
            .assert_eq(&hex_fmt(&x_p));
        }
    }

    #[test]
    #[ignore = "underspecified algorithm"]
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
                47 dd 7f 2c 11 f3 05 e6 97 40 95 e3 c8 61 2f 6e
                8d 09 bb ea 63 ef be 8d 84 55 8f cb f5 28 81 37"#]]
        .assert_eq(&hex_fmt(&areion512_md(&data)));
    }

    #[test]
    #[ignore = "underspecified algorithm"]
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
                61 17 b5 9f 30 25 cd 4e 66 8b dc b3 66 bd 89 b9
                06 0e 8d cf 67 0c bf 43 08 a8 96 86 8e bc c6 fc7"#]]
        .assert_eq(&hex_fmt(&areion512_md(&data)));
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
