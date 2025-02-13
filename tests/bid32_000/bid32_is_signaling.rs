use super::*;

#[test]
fn _0001() {
  assert!(!bid32_is_signaling(d32("QNaN")));
}

#[test]
fn _0002() {
  assert!(bid32_is_signaling(d32("SNaN")));
}
