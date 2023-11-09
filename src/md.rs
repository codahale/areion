use core::{fmt, slice};

use crate::intrinsics::*;

use digest::block_buffer::Eager;
use digest::core_api::{
    Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, FixedOutputCore, UpdateCore,
};
use digest::crypto_common::AlgorithmName;
use digest::generic_array::GenericArray;
use digest::typenum::{Unsigned, U32};
use digest::{HashMarker, Output, OutputSizeUser, Reset};
use hex_literal::hex;

#[derive(Debug, Clone)]
struct State(AesBlock, AesBlock);

impl Default for State {
    fn default() -> Self {
        Self(
            load(&hex!("6a09e667bb67ae853c6ef372a54ff53a")),
            load(&hex!("510e527f9b05688c1f83d9ab5be0cd19")),
        )
    }
}

impl State {
    fn compress(&mut self, blocks: &[GenericArray<u8, U32>]) {
        let Self(mut h0, mut h1) = self;
        for block in blocks {
            let (m0, m1) = (load(&block[..16]), load(&block[16..]));
            (h0, h1) = crate::areion512_dm(m0, m1, h0, h1);
        }
        *self = Self(h0, h1);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Core {
    state: State,
    block_len: u64,
}

impl HashMarker for Core {}

impl BlockSizeUser for Core {
    type BlockSize = U32;
}

impl BufferKindUser for Core {
    type BufferKind = Eager;
}

impl OutputSizeUser for Core {
    type OutputSize = U32;
}

impl UpdateCore for Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u64;
        self.state.compress(blocks);
    }
}

impl FixedOutputCore for Core {
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        let bs = Self::BlockSize::U64;
        let bit_len = 8 * (buffer.get_pos() as u64 + bs * self.block_len);
        buffer.len64_padding_be(bit_len, |b| self.state.compress(slice::from_ref(b)));

        store(&mut out[..16], self.state.0);
        store(&mut out[16..], self.state.1);
    }
}

impl Reset for Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}

impl AlgorithmName for Core {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Areion512-MD")
    }
}

pub type Areion512Md = CoreWrapper<Core>;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tests::hex_fmt;

    use digest::Digest;
    use expect_test::expect;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn different_inputs_yield_different_digests(a: Vec<u8>, b: Vec<u8>) -> bool {
        let aa = Areion512Md::new().chain_update(&a).finalize();
        let bb = Areion512Md::new().chain_update(&b).finalize();
        aa == bb || a != b
    }

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
}
