use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  assert!(bid128_is_normal(d128("1")));
}

#[test]
fn _0002() {
  assert!(!bid128_is_normal(bid128_nan("0")));
}

#[test]
fn _0003() {
  assert!(!bid128_is_normal(bid128_infinite()));
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  let value = bid128_from_string("1E-6177", RM_NEAREST_EVEN, &mut flags);
  eq("+0E-6176", value);

  let a = d128("1E-6176");
  let b = d128("10");

  eq("+1E-6176", a);
  eq("+10E+0", b);

  let value = bid128_div(a, b, RM_NEAREST_EVEN, &mut flags);
  eq("+0E-6176", value);

  assert!(!bid128_is_normal(bid128_infinite()));
}
