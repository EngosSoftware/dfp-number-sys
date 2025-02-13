use super::*;

#[test]
fn _0001() {
  assert!(!bid32_is_subnormal(d32("1")));
}

#[test]
fn _0002() {
  assert!(bid32_is_subnormal(bid32_positive_subnormal()));
}
