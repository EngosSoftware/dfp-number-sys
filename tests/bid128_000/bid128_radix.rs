use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  assert_eq!(10, bid128_radix(d128("2")));
}

#[test]
fn _0002() {
  assert_eq!(10, bid128_radix(d128("2.498537")));
}

#[test]
fn _0003() {
  assert_eq!(10, bid128_radix(d128("Inf")));
}
