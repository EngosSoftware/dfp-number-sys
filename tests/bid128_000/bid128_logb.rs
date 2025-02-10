use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("-Inf", bid128_logb(d128("0"), &mut flags));
  eqf(FB_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_logb(d128("1"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+1E+0", bid128_logb(d128("10.0"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+2E+0", bid128_logb(d128("256.0"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("-4E+0", bid128_logb(d128("0.0001"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+Inf", bid128_logb(d128("+Inf"), &mut flags));
  eqf(FB_CLEAR, flags);
}
