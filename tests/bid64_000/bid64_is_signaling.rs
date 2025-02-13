use super::*;

#[test]
fn _0001() {
  assert!(!bid64_is_signaling(d64("QNaN")));
}

#[test]
fn _0002() {
  assert!(bid64_is_signaling(d64("SNaN")));
}
