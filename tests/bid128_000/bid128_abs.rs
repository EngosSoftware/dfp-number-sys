use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  eq("+2E+0", bid128_abs(bid128_from_int32(2)));
}

#[test]
fn _0002() {
  eq("+2E+0", bid128_abs(bid128_from_int32(-2)));
}
