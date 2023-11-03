use core::arch::x86_64::*;

pub use core::arch::x86_64::__m128i as Block;

#[inline]
pub fn zero() -> Block {
    unsafe { _mm_setzero_si128() }
}

#[inline]
pub fn load_32x4(a: u32, b: u32, c: u32, d: u32) -> Block {
    unsafe {
        _mm_set_epi32(
            d.try_into().unwrap(),
            c.try_into().unwrap(),
            b.try_into().unwrap(),
            a.try_into().unwrap(),
        )
    }
}

#[inline]
pub fn load(bytes: &[u8]) -> Block {
    unsafe { _mm_loadu_si128(bytes.as_ptr() as *const __m128i) }
}

#[inline]
pub fn store(bytes: &mut [u8], block: Block) {
    unsafe { _mm_storeu_si128(bytes.as_mut_ptr() as *mut __m128i, block) };
}

#[inline]
pub fn xor(a: Block, b: Block) -> Block {
    unsafe { _mm_xor_si128(a, b) }
}

#[inline]
pub fn xor3(a: Block, b: Block, c: Block) -> Block {
    unsafe { _mm_xor_si128(a, _mm_xor_si128(b, c)) }
}

#[inline]
pub fn enc(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesenc_si128(state, round_key) }
}

#[inline]
pub fn enc_last(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesenclast_si128(state, round_key) }
}

#[inline]
pub fn dec_last(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesdeclast_si128(state, round_key) }
}

#[inline]
pub fn inv_mix(state: Block) -> Block {
    unsafe { _mm_aesimc_si128(state) }
}
