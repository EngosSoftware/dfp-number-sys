use super::*;

#[test]
fn _0001() {
  assert!(bid64_is_finite(d64("-1")));
}

#[test]
fn _0002() {
  assert!(!bid64_is_finite(d64("NaN")));
}

#[test]
fn _0003() {
  assert!(!bid64_is_finite(d64("QNaN")));
}

#[test]
fn _0004() {
  assert!(!bid64_is_finite(d64("SNaN")));
}

#[test]
fn _0005() {
  assert!(!bid64_is_finite(d64("+Inf")));
}

#[test]
fn _0006() {
  assert!(!bid64_is_finite(d64("+Infinite")));
}

#[test]
fn _0007() {
  assert!(!bid64_is_finite(d64("-Inf")));
}

#[test]
fn _0008() {
  assert!(!bid64_is_finite(d64("-Infinite")));
}
