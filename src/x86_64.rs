use core::arch::x86_64::*;

pub use core::arch::x86_64::__m128i as AesBlock;

#[inline]
pub fn zero() -> AesBlock {
    unsafe { _mm_setzero_si128() }
}

#[inline]
pub fn load_64x2(a: u64, b: u64) -> AesBlock {
    unsafe { _mm_set_epi64x(b as i64, a as i64) }
}

#[inline]
pub fn load_32x4(a: u32, b: u32, c: u32, d: u32) -> AesBlock {
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
pub fn load(bytes: &[u8]) -> AesBlock {
    unsafe { _mm_loadu_si128(bytes.as_ptr() as *const __m128i) }
}

#[inline]
pub fn store(bytes: &mut [u8], block: AesBlock) {
    unsafe { _mm_storeu_si128(bytes.as_mut_ptr() as *mut __m128i, block) };
}

#[inline]
pub fn xor(a: AesBlock, b: AesBlock) -> AesBlock {
    unsafe { _mm_xor_si128(a, b) }
}

#[inline]
pub fn xor3(a: AesBlock, b: AesBlock, c: AesBlock) -> AesBlock {
    unsafe { _mm_xor_si128(a, _mm_xor_si128(b, c)) }
}

#[inline]
pub fn enc(state: AesBlock, round_key: AesBlock) -> AesBlock {
    unsafe { _mm_aesenc_si128(state, round_key) }
}

#[inline]
pub fn enc_last(state: AesBlock, round_key: AesBlock) -> AesBlock {
    unsafe { _mm_aesenclast_si128(state, round_key) }
}

#[inline]
pub fn dec_last(state: AesBlock, round_key: AesBlock) -> AesBlock {
    unsafe { _mm_aesdeclast_si128(state, round_key) }
}

#[inline]
pub fn inv_mix(state: AesBlock) -> AesBlock {
    unsafe { _mm_aesimc_si128(state) }
}
