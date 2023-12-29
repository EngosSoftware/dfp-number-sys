use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  assert!(bid128_is_zero(BID128_ZERO));
  eq("+0E+0", BID128_ZERO);
}

#[test]
fn _0002() {
  assert!(bid128_is_zero(BID128_MINUS_ZERO));
  eq("-0E+0", BID128_MINUS_ZERO);
}

#[test]
fn _0003() {
  eq("+1E+0", BID128_ONE);
}

#[test]
fn _0004() {
  eq("-1E+0", BID128_MINUS_ONE);
}

#[test]
fn _0005() {
  eq("+2E+0", BID128_TWO);
}

#[test]
fn _0006() {
  eq("-2E+0", BID128_MINUS_TWO);
}

#[test]
fn _0007() {
  eq("+1E-1", BID128_ONE_TENTH);
}

#[test]
fn _0008() {
  eq("+Inf", BID128_INF);
}

#[test]
fn _0009() {
  eq("-Inf", BID128_MINUS_INF);
}

#[test]
fn _0010() {
  eq("+1000000000E+0", BID128_BILLION);
}

#[test]
fn _0011() {
  eq("-1000000000E+0", BID128_MINUS_BILLION);
}
