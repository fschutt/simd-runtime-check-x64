//! This is a (nightly-only) library that
//! provides an check at runtime to see if various SIMD
//! features are supported.
//!
//! The code has been taken from `parched/runtime-target-feature-rs`, see
//! https://github.com/parched/runtime-target-feature-rs/blob/master/rt/src/x86.rs
//!
//! I have only seperated it into a crate because the `runtime-target-feature-rs`
//! crate didn't provide me with enough control to actually choose between
//! SIMD and non-SIMD versions of functions at runtime.

#![feature(asm)]

#[macro_use]
extern crate lazy_static;

lazy_static! { static ref FEATURES: [u32; 7] = unsafe {
    let highest_cpuid: u32;
    let features_0: u32;
    let features_1: u32;
    let features_2: u32;
    let features_3: u32;
    let features_4: u32;

    let extended_highest_cpuid: u32;
    let features_5: u32;
    let features_6: u32;

    asm!("cpuid" : "={eax}"(highest_cpuid) : "{eax}"(0) : "ebx", "ecx", "edx");

    if highest_cpuid >= 1 {
        asm!("cpuid" : "={ecx}"(features_1), "={edx}"(features_0) : "{eax}"(1) : "eax", "ebx");
        if highest_cpuid >= 7 {
            asm!("cpuid":
                 "={ebx}"(features_2), "={ecx}"(features_3), "={edx}"(features_4):
                 "{eax}"(7), "{ecx}"(0):
                 "eax");
        } else {
            features_2 = 0;
            features_3 = 0;
            features_4 = 0;
        }

    } else {
        features_0 = 0;
        features_1 = 0;
        features_2 = 0;
        features_3 = 0;
        features_4 = 0;
    }

    asm!("cpuid" : "={eax}"(extended_highest_cpuid) : "{eax}"(0) : "ebx", "ecx", "edx");

    if extended_highest_cpuid >= 0x80000001u32 {
        asm!("cpuid":
             "={ecx}"(features_6), "={edx}"(features_5):
             "{eax}"(0x80000001u32):
             "eax", "ebx");
    } else {
        features_5 = 0;
        features_6 = 0;
    }

    [features_0, features_1, features_2, features_3, features_4, features_5, features_6]
};
}

pub fn has_avx() -> bool {
    test_bit(1, 28)
}

pub fn has_avx2() -> bool {
    test_bit(2, 5)
}

pub fn has_bmi() -> bool {
    test_bit(2, 3)
}

pub fn has_bmi2() -> bool {
    test_bit(2, 8)
}

pub fn has_sse() -> bool {
    test_bit(0, 25)
}

pub fn has_sse2() -> bool {
    test_bit(0, 26)
}

pub fn has_sse3() -> bool {
    test_bit(1, 0)
}

pub fn has_sse4_1() -> bool {
    test_bit(1, 19)
}

pub fn has_sse4_2() -> bool {
    test_bit(1, 20)
}

pub fn has_ssse3() -> bool {
    test_bit(1, 9)
}

pub fn has_tbm() -> bool {
    test_bit(6, 21)
}

pub fn has_lzcnt() -> bool {
    test_bit(6, 55)
}

pub fn has_popcnt() -> bool {
    test_bit(1, 23)
}

pub fn has_sse4a() -> bool {
    test_bit(6, 6)
}

pub fn has_rdrnd() -> bool {
    test_bit(1, 30)
}

pub fn has_rdseed() -> bool {
    test_bit(2, 18)
}

pub fn has_fma() -> bool {
    test_bit(1, 12)
}

fn test_bit(word: usize, bit: usize) -> bool {
    FEATURES[word] & (1u32 << bit) != 0u32
}