use super::*;

#[test]
fn _0001() {
  assert!(bid128_is_normal(d128("1")));
}

#[test]
fn _0002() {
  assert!(!bid128_is_normal(bid128_nan("0")));
}

#[test]
fn _0003() {
  assert!(!bid128_is_normal(bid128_infinite()));
}

#[test]
fn _0004() {
  assert!(!bid128_is_normal(bid128_positive_subnormal()));
}

#[test]
fn _0005() {
  assert!(!bid128_is_normal(bid128_negative_subnormal()));
}
