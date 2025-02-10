use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("-Inf", bid128_log10(d128("0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_log10(d128("1"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+3E+0", bid128_log10(d128("1000.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+1E+0", bid128_log10(d128("10.0"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+1372543800759070330613848971932462E-33", bid128_log10(d128("23.58"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+Inf", bid128_log10(d128("+Inf"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
