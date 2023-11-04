use core::{fmt, slice};

#[cfg(target_arch = "aarch64")]
use crate::aarch64::{Block as AesBlock, *};

#[cfg(target_arch = "x86_64")]
use crate::x86_64::{Block as AesBlock, *};

use digest::block_buffer::Eager;
use digest::core_api::{
    Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, FixedOutputCore, UpdateCore,
};
use digest::crypto_common::AlgorithmName;
use digest::generic_array::GenericArray;
use digest::typenum::{Unsigned, U32, U64};
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
            let (m0, m1) = (load(&block[..16]), load(&block[16..32]));
            let (x0, x1) = crate::areion512_dm(m0, m1, h0, h1);
            (h0, h1) = (xor(h0, x0), xor(h1, x1));
        }
        *self = Self(h0, h1);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Core {
    state: State,
    block_len: u128,
}

impl HashMarker for Core {}

impl BlockSizeUser for Core {
    type BlockSize = U32;
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
