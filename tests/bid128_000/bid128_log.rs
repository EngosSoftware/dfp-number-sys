use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("-Inf", bid128_log(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_log(d128("1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq(
    "+1000000000000000000000000000000014E-33",
    bid128_log(d128("2.7182818284590452353602874713527"), RM_NEAREST_EVEN, &mut flags),
  );
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+2302585092994045684017991454684364E-33", bid128_log(d128("10.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+Inf", bid128_log(d128("+Inf"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
