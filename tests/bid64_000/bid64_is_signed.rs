use super::*;

#[test]
fn _0001() {
  assert!(bid64_is_signed(d64("-2")));
}

#[test]
fn _0002() {
  assert!(!bid64_is_signed(d64("2")));
}
