use core::fmt;

use crate::intrinsics::*;

use digest::block_buffer::Lazy;
use digest::core_api::{
    Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, CtVariableCoreWrapper,
    RtVariableCoreWrapper, TruncSide, UpdateCore, VariableOutputCore,
};
use digest::crypto_common::AlgorithmName;
use digest::generic_array::GenericArray;
use digest::typenum::{Unsigned, U64};
use digest::{HashMarker, Output, OutputSizeUser, Reset};

#[derive(Debug, Clone)]
struct State {
    /// The 512-bit hash state.
    h: (AesBlock, AesBlock, AesBlock, AesBlock),
    /// The 512-bit tweak.
    t: (AesBlock, AesBlock, AesBlock, AesBlock),
    /// The message length counter, in bits.
    ctr: u128,
}

impl State {
    fn new(output_size: usize) -> State {
        // The hash state is initialized with the SHA2-512 IV constants, with the fourth word XORed
        // with the output size in bits.
        State {
            h: (
                // SHA2-512 IV constants
                load_64x2(0x6a09e667f3bcc908, 0xbb67ae8584caa73b),
                load_64x2(0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1),
                load_64x2(0x510e527fade682d1, 0x9b05688c2b3e6c1f),
                load_64x2(0x1f83d9abfb41bd6b, 0x5be0cd19137e2179),
            ),
            t: (
                // SHA2-512-256 IV constants
                load_64x2(0x22312194fc2bf72c, 0x9f555fa3c84c64c2),
                load_64x2(0x2393b86b6f53b151, 0x963877195940eabd),
                load_64x2(0x96283ee2a88effe3, 0xbe5e1e2553863992),
                load_64x2(0x2b0199fc2c85b8aa, 0x0eb72ddc81c52ca2 ^ output_size as u64),
            ),
            ctr: 0,
        }
    }
}

impl State {
    fn compress(&mut self, blocks: &[GenericArray<u8, U64>], bit_len: u64) {
        let Self {
            h: (mut h0, mut h1, mut h2, mut h3),
            t: (t0, t1, t2, t3),
            mut ctr,
        } = *self;

        for block in blocks {
            // Increment the bit counter *before* compressing the block. This eliminates the need
            // for finalization-specific flags, as the output of compressing the final block of N
            // bits will be dependent on the value of ctr+N.
            ctr += bit_len as u128;

            // Load the message block into four words.
            let (m0, m1, m2, m3) = (
                load(&block[..16]),
                load(&block[16..32]),
                load(&block[32..48]),
                load(&block[48..]),
            );

            // C(H, T, M, #bits) = P(H ^ T ^ M ^ #bits) ^ H ^ T
            let (x0, x1, x2, x3) = (xor(h0, t0), xor(h1, t1), xor(h2, t2), xor(h3, t3));
            let (y0, y1, y2, y3) = crate::areion512(
                xor(x0, m0),
                xor(x1, m1),
                xor(x2, m2),
                // Only include the counter as an input to the permutation. This avoids a
                // Streebog-type situation in which attackers have control of some of the bits of
                // the output of a block's compression.
                xor3(x3, m3, load(&ctr.to_le_bytes())),
            );
            (h0, h1, h2, h3) = (xor(x0, y0), xor(x1, y1), xor(x2, y2), xor(x3, y3));
        }

        // Update the hash state and counter.
        self.h = (h0, h1, h2, h3);
        self.ctr = ctr;
    }
}

#[derive(Debug, Clone)]
pub struct Core {
    state: State,
    output_size: usize,
}

impl HashMarker for Core {}

impl BlockSizeUser for Core {
    type BlockSize = U64;
}

impl BufferKindUser for Core {
    type BufferKind = Lazy;
}

impl OutputSizeUser for Core {
    type OutputSize = U64;
}

impl UpdateCore for Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        // Update the state with the compression function, using the length of a full block in bits
        // to update the counter.
        self.state.compress(blocks, Self::BlockSize::U64 * 8);
    }
}

impl VariableOutputCore for Core {
    const TRUNC_SIDE: TruncSide = TruncSide::Left;

    fn new(output_size: usize) -> Result<Self, digest::InvalidOutputSize> {
        if !(0 < output_size && output_size <= 64) {
            return Err(digest::InvalidOutputSize);
        }
        Ok(Core {
            state: State::new(output_size),
            output_size,
        })
    }

    fn finalize_variable_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        // Update the state with the compression function, using the length of the remaining data
        // in bits to update the counter.
        let bit_len = buffer.get_pos() as u64 * 8;
        let padded = [*buffer.pad_with_zeros()];
        self.state.compress(padded.as_slice(), bit_len);

        // Use the hash state as the digest, truncating as needed.
        let mut tmp = [0u8; 64];
        store(&mut tmp[..16], self.state.h.0);
        store(&mut tmp[16..32], self.state.h.1);
        store(&mut tmp[32..48], self.state.h.2);
        store(&mut tmp[48..], self.state.h.3);
        let n = out.len();
        out.copy_from_slice(&tmp[..n]);
    }
}

impl Reset for Core {
    #[inline]
    fn reset(&mut self) {
        self.state = State::new(self.output_size);
    }
}

impl AlgorithmName for Core {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Areion512-HAIFA")
    }
}

pub type AreionHaifaVar = RtVariableCoreWrapper<Core>;
pub type AreionHaifaCore<OutSize> = CtVariableCoreWrapper<Core, OutSize>;
pub type AreionHaifa<OutSize> = CoreWrapper<AreionHaifaCore<OutSize>>;
pub type AreionHaifa512 = AreionHaifa<U64>;

#[cfg(test)]
mod tests {
    use digest::Digest;

    use super::*;

    #[test]
    fn round_trip() {
        AreionHaifa512::new()
            .chain_update([8u8; 64])
            .chain_update(b"this is a potato")
            .finalize();
    }
}
