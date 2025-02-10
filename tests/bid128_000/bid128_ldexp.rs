use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_ldexp(d128("0"), 0, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E-1", bid128_ldexp(d128("0"), -1, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+0E+1", bid128_ldexp(d128("0"), 1, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+15E-1", bid128_ldexp(d128("1.5"), 0, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+15E+0", bid128_ldexp(d128("1.5"), 1, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+15E+1", bid128_ldexp(d128("1.5"), 2, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  eq("+15E+7", bid128_ldexp(d128("1.5"), 8, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = FB_CLEAR;
  eq("+15E-9", bid128_ldexp(d128("1.5"), -8, RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
