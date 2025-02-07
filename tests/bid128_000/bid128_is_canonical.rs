use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  assert!(bid128_is_canonical(d128("-1")));
}

#[test]
fn _0002() {
  assert!(bid128_is_canonical(d128("NaN")));
}

#[test]
fn _0003() {
  assert!(bid128_is_canonical(d128("+Inf")));
}

#[test]
fn _0004() {
  assert!(bid128_is_canonical(d128("-Inf")));
}
