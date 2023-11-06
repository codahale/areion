use core::{fmt, slice};

#[cfg(target_arch = "aarch64")]
use crate::aarch64::*;

#[cfg(target_arch = "x86_64")]
use crate::x86_64::*;

use digest::block_buffer::Eager;
use digest::core_api::{
    Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, FixedOutputCore, UpdateCore,
};
use digest::crypto_common::AlgorithmName;
use digest::generic_array::GenericArray;
use digest::typenum::{Unsigned, U64};
use digest::{HashMarker, Output, OutputSizeUser, Reset};

#[derive(Debug, Clone)]
struct State(AesBlock, AesBlock, AesBlock, AesBlock);

impl Default for State {
    fn default() -> Self {
        Self(
            load_64x2(0x6a09e667f3bcc908u64, 0xbb67ae8584caa73bu64),
            load_64x2(0x3c6ef372fe94f82bu64, 0xa54ff53a5f1d36f1u64),
            load_64x2(0x510e527fade682d1u64, 0x9b05688c2b3e6c1fu64),
            load_64x2(0x1f83d9abfb41bd6bu64, 0x5be0cd19137e2179u64),
        )
    }
}

impl State {
    fn compress(&mut self, blocks: &[GenericArray<u8, U64>]) {
        let Self(mut h0, mut h1, mut h2, mut h3) = self;
        for block in blocks {
            let (m0, m1, m2, m3) = (
                load(&block[..16]),
                load(&block[16..32]),
                load(&block[32..48]),
                load(&block[48..]),
            );

            // SEM(K, P) = F(P ^ K) ^ K
            // MMO(H, M) = SEM(H, M) ^ M
            // SEM_MMO(H, M) = F(M ^ H) ^ H ^ M
            h0 = xor(h0, m0);
            h1 = xor(h1, m1);
            h2 = xor(h2, m2);
            h3 = xor(h3, m3);
            let (x0, x1, x2, x3) = crate::areion512(h0, h1, h2, h3);
            h0 = xor3(x0, h0, m0);
            h1 = xor3(x1, h1, m1);
            h2 = xor3(x2, h2, m2);
            h3 = xor3(x3, h3, m3);
        }
        *self = Self(h0, h1, h2, h3);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Core {
    state: State,
    block_len: u128,
}

impl HashMarker for Core {}

impl BlockSizeUser for Core {
    type BlockSize = U64;
}

impl BufferKindUser for Core {
    type BufferKind = Eager;
}

impl OutputSizeUser for Core {
    type OutputSize = U64;
}

impl UpdateCore for Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.block_len += blocks.len() as u128;
        self.state.compress(blocks);
    }
}

impl FixedOutputCore for Core {
    fn finalize_fixed_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        let bs = Self::BlockSize::U64 as u128;
        let bit_len = 8 * (buffer.get_pos() as u128 + bs * self.block_len);
        buffer.len128_padding_be(bit_len, |b| self.state.compress(slice::from_ref(b)));

        store(&mut out[..16], self.state.0);
        store(&mut out[16..32], self.state.1);
        store(&mut out[32..48], self.state.2);
        store(&mut out[48..], self.state.3);
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
        f.write_str("Areion512-MMO")
    }
}

pub type Areion512Mmo = CoreWrapper<Core>;
