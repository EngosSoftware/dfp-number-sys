use super::*;

#[test]
fn _0001() {
  assert!(bid32_is_finite(d32("-1")));
}

#[test]
fn _0002() {
  assert!(!bid32_is_finite(d32("NaN")));
}

#[test]
fn _0003() {
  assert!(!bid32_is_finite(d32("QNaN")));
}

#[test]
fn _0004() {
  assert!(!bid32_is_finite(d32("SNaN")));
}

#[test]
fn _0005() {
  assert!(!bid32_is_finite(d32("+Inf")));
}

#[test]
fn _0006() {
  assert!(!bid32_is_finite(d32("+Infinite")));
}

#[test]
fn _0007() {
  assert!(!bid32_is_finite(d32("-Inf")));
}

#[test]
fn _0008() {
  assert!(!bid32_is_finite(d32("-Infinite")));
}
