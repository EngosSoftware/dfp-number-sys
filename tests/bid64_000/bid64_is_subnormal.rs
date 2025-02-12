use super::*;

#[test]
fn _0001() {
  assert!(!bid64_is_subnormal(d64("1")));
}

#[test]
fn _0002() {
  assert!(bid64_is_subnormal(bid64_positive_subnormal()));
}
