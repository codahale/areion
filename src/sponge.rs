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
struct State(AesBlock, AesBlock, AesBlock, AesBlock);

impl Default for State {
    fn default() -> Self {
        Self(
            load(&hex!("08c9bcf367e6096a3ba7ca8485ae67bb")),
            load(&hex!("2bf894fe72f36e3cf1361d5f3af54fa5")),
            load(&hex!("d182e6ad7f520e511f6c3e2b8c68059b")),
            load(&hex!("6bbd41fbabd9831f79217e1319cde05b")),
        )
    }
}

impl State {
    fn compress(&mut self, blocks: &[GenericArray<u8, U32>]) {
        let Self(mut h0, mut h1, mut h2, mut h3) = self;
        for block in blocks {
            let (m0, m1) = (load(&block[..16]), load(&block[16..]));
            (h0, h1, h2, h3) = crate::areion512(xor(h0, m0), xor(h1, m1), h2, h3);
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
        f.write_str("Areion512-256-Sponge")
    }
}

pub type Areion256Sponge = CoreWrapper<Core>;
