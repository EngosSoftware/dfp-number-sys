use super::*;

#[test]
fn _0001() {
  assert!(!bid32_is_nan(d32("0")));
}

#[test]
fn _0002() {
  assert!(bid32_is_nan(bid32_nan("0")));
}
