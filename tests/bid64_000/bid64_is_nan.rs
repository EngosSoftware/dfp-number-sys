use super::*;

#[test]
fn _0001() {
  assert!(!bid64_is_nan(d64("0")));
}

#[test]
fn _0002() {
  assert!(bid64_is_nan(bid64_nan("0")));
}
