## simd-runtime-check-x64

[Crates.io](https://crates.io/crates/simd-runtime-check-x64) | [Documentation](https://docs.rs/simd-runtime-check-x64)

This is a (nightly-only) library that provides an check at runtime to
see if various SIMD features are supported.

The code has been taken from `parched/runtime-target-feature-rs`, see
https://github.com/parched/runtime-target-feature-rs/blob/master/rt/src/x86.rs

I have only seperated it into a crate because the `runtime-target-feature-rs`
crate didn't provide me with enough control to actually choose between
SIMD and non-SIMD versions of functions at runtime.