use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  assert!(!bid128_is_subnormal(d128("1")));
}

#[test]
fn _0002() {
  assert!(bid128_is_subnormal(subnormal()));
}
