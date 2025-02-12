use super::*;

#[test]
fn _0001() {
  assert!(bid128_is_finite(bid128_from_int32(-1)));
}

#[test]
fn _0002() {
  assert!(!bid128_is_finite(d128("NaN")));
}

#[test]
fn _0003() {
  assert!(!bid128_is_finite(d128("QNaN")));
}

#[test]
fn _0004() {
  assert!(!bid128_is_finite(d128("SNaN")));
}

#[test]
fn _0005() {
  assert!(!bid128_is_finite(d128("+Inf")));
}

#[test]
fn _0006() {
  assert!(!bid128_is_finite(d128("+Infinite")));
}

#[test]
fn _0007() {
  assert!(!bid128_is_finite(d128("-Inf")));
}

#[test]
fn _0008() {
  assert!(!bid128_is_finite(d128("-Infinite")));
}
