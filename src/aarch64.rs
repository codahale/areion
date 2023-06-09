use core::arch::aarch64::*;
use core::arch::asm;

pub use core::arch::aarch64::uint8x16_t as Block;

#[inline(always)]
pub fn load(bytes: &[u8]) -> uint8x16_t {
    unsafe { vld1q_u8(bytes.as_ptr()) }
}

#[inline(always)]
pub fn store(bytes: &mut [u8], block: uint8x16_t) {
    unsafe { vst1q_u8(bytes.as_mut_ptr(), block) };
}

#[inline(always)]
pub fn xor(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    unsafe { veorq_u8(a, b) }
}

#[inline(always)]
pub fn xor3(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    // TODO replace with veor3q_u8 intrinsic when that's stable
    #[target_feature(enable = "sha3")]
    unsafe fn veor3q_u8(mut a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
        asm!(
            "EOR3 {0:v}.16B, {0:v}.16B, {1:v}.16B, {2:v}.16B",
            inlateout(vreg) a, in(vreg) b, in(vreg) c,
            options(pure, nomem, nostack, preserves_flags)
        );
        a
    }
    unsafe { veor3q_u8(a, b, c) }
}

#[inline(always)]
pub fn enc(state: uint8x16_t, round_key: uint8x16_t) -> uint8x16_t {
    // TODO replace with vaeseq_u8 and vaesmcq_u8 instrinsics when that's stable
    #[target_feature(enable = "aes")]
    unsafe fn vaeseq_u8_and_vaesmcq_u8(mut state: uint8x16_t) -> uint8x16_t {
        asm!(
            "AESE {0:v}.16B, {1:v}.16B",
            "AESMC {0:v}.16B, {0:v}.16B",
            inlateout(vreg) state, in(vreg) 0,
            options(pure, nomem, nostack, preserves_flags)
        );
        state
    }
    unsafe { xor(vaeseq_u8_and_vaesmcq_u8(state), round_key) }
}

#[inline(always)]
pub fn enc_last(state: uint8x16_t, round_key: uint8x16_t) -> uint8x16_t {
    // TODO replace with vaeseq_u8 instrinsics when that's stable
    #[target_feature(enable = "aes")]
    unsafe fn vaeseq_u8(mut state: uint8x16_t) -> uint8x16_t {
        asm!(
            "AESE {0:v}.16B, {1:v}.16B",
            inlateout(vreg) state, in(vreg) 0,
            options(pure, nomem, nostack, preserves_flags)
        );
        state
    }
    unsafe { xor(vaeseq_u8(state), round_key) }
}

#[inline(always)]
pub fn dec_last(state: uint8x16_t, round_key: uint8x16_t) -> uint8x16_t {
    // TODO replace with vaeseq_u8 instrinsics when that's stable
    #[target_feature(enable = "aes")]
    unsafe fn vaesdq_u8(mut state: uint8x16_t) -> uint8x16_t {
        asm!(
            "AESD {0:v}.16B, {1:v}.16B",
            inlateout(vreg) state, in(vreg) 0,
            options(pure, nomem, nostack, preserves_flags)
        );
        state
    }
    unsafe { xor(vaesdq_u8(state), round_key) }
}

#[inline(always)]
pub fn inv_mix(state: uint8x16_t) -> uint8x16_t {
    // TODO replace with vaesimcq_u8 instrinsics when that's stable
    #[target_feature(enable = "aes")]
    unsafe fn vaesimcq_u8(mut state: uint8x16_t) -> uint8x16_t {
        asm!(
            "AESIMC {0:v}.16B, {0:v}.16B",
            inlateout(vreg) state,
            options(pure, nomem, nostack, preserves_flags)
        );
        state
    }
    unsafe { vaesimcq_u8(state) }
}
