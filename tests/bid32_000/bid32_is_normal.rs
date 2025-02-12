use super::*;

#[test]
fn _0001() {
  assert!(bid32_is_normal(d32("1")));
}

#[test]
fn _0002() {
  assert!(!bid32_is_normal(bid32_nan("0")));
}

#[test]
fn _0003() {
  assert!(!bid32_is_normal(bid32_infinite()));
}

#[test]
fn _0004() {
  assert!(!bid32_is_normal(bid32_positive_subnormal()));
}

#[test]
fn _0005() {
  assert!(!bid32_is_normal(bid32_negative_subnormal()));
}
