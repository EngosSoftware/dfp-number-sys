use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+2256E-3", bid128_max_num(d128("1.234"), d128("2.256"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1234E-3", bid128_max_num(d128("1.234"), d128("NaN"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+2256E-3", bid128_max_num(d128("NaN"), d128("2.256"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_max_num(d128("1.234"), d128("SNaN"), &mut flags));
  eqf(FB_INVALID, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+NaN", bid128_max_num(d128("SNaN"), d128("2.256"), &mut flags));
  eqf(FB_INVALID, flags);
}
