use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+208717E-5", bid128_fdim(d128("5.43557"), d128("3.3484"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+0E+0", bid128_fdim(d128("3.3484"), d128("5.43557"), RM_NEAREST_EVEN, &mut flags));
  eqf(FB_CLEAR, flags);
}
