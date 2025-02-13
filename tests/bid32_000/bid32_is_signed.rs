use super::*;

#[test]
fn _0001() {
  assert!(bid32_is_signed(d32("-2")));
}

#[test]
fn _0002() {
  assert!(!bid32_is_signed(d32("2")));
}
