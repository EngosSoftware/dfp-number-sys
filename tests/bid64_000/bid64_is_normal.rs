use super::*;

#[test]
fn _0001() {
  assert!(bid64_is_normal(d64("1")));
}

#[test]
fn _0002() {
  assert!(!bid64_is_normal(bid64_nan("0")));
}

#[test]
fn _0003() {
  assert!(!bid64_is_normal(bid64_infinite()));
}

#[test]
fn _0004() {
  assert!(!bid64_is_normal(bid64_positive_subnormal()));
}

#[test]
fn _0005() {
  assert!(!bid64_is_normal(bid64_negative_subnormal()));
}
