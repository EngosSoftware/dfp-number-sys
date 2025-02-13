use super::*;

#[test]
fn _0001() {
  assert!(bid32_is_zero(d32("0")));
}

#[test]
fn _0002() {
  assert!(!bid32_is_zero(d32("-1")));
}

#[test]
fn _0003() {
  assert!(!bid32_is_zero(d32("1")));
}
