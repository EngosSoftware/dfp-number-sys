//! # Rust bindings for Intel® Decimal Floating-Point Math Library v2.2

#[cfg(all(not(feature = "call-by-reference"), not(feature = "global-rounding"), not(feature = "global-exception-flags")))]
mod bid128_000;
mod common;

#[cfg(all(not(feature = "call-by-reference"), not(feature = "global-rounding"), not(feature = "global-exception-flags")))]
pub use bid128_000::*;
pub use common::*;
