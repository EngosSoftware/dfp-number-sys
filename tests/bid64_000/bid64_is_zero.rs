use super::*;

#[test]
fn _0001() {
  assert!(bid64_is_zero(d64("0")));
}

#[test]
fn _0002() {
  assert!(!bid64_is_zero(d64("-1")));
}

#[test]
fn _0003() {
  assert!(!bid64_is_zero(d64("1")));
}
