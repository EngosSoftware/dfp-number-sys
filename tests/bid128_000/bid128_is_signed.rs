use super::*;

#[test]
fn _0001() {
  assert!(bid128_is_signed(d128("-2")));
}

#[test]
fn _0002() {
  assert!(!bid128_is_signed(d128("2")));
}
