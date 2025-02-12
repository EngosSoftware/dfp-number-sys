use super::*;

#[test]
fn _0001() {
  assert!(!bid128_is_signaling(d128("QNaN")));
}

#[test]
fn _0002() {
  assert!(bid128_is_signaling(d128("SNaN")));
}
