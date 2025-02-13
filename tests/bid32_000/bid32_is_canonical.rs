use super::*;

#[test]
fn _0001() {
  assert!(bid32_is_canonical(d32("-1")));
}

#[test]
fn _0002() {
  assert!(bid32_is_canonical(d32("NaN")));
}

#[test]
fn _0003() {
  assert!(bid32_is_canonical(d32("+Inf")));
}

#[test]
fn _0004() {
  assert!(bid32_is_canonical(d32("-Inf")));
}
