use super::*;

#[test]
fn _0001() {
  assert!(!bid128_is_nan(d128("0")));
}

#[test]
fn _0002() {
  assert!(bid128_is_nan(bid128_nan("0")));
}
