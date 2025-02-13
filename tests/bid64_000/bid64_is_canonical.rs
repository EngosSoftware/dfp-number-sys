use super::*;

#[test]
fn _0001() {
  assert!(bid64_is_canonical(d64("-1")));
}

#[test]
fn _0002() {
  assert!(bid64_is_canonical(d64("NaN")));
}

#[test]
fn _0003() {
  assert!(bid64_is_canonical(d64("+Inf")));
}

#[test]
fn _0004() {
  assert!(bid64_is_canonical(d64("-Inf")));
}
