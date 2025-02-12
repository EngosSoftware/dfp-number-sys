use super::*;

use dfp_number_sys::bid64_000::bid64_to_string;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let bid64 = bid128_to_bid64(d128("0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("+0E+0", bid64_to_string(bid64, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let bid64 = bid128_to_bid64(d128("-0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("-0E+0", bid64_to_string(bid64, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let bid64 = bid128_to_bid64(d128("+9999999999999999999999999999999999E+6111"), RND_TOWARD_ZERO, &mut flags);
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
  flags = EXE_CLEAR;
  assert_eq!("+9999999999999999E+369", bid64_to_string(bid64, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  let bid64 = bid128_to_bid64(d128("-9999999999999999999999999999999999E+6111"), RND_TOWARD_ZERO, &mut flags);
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
  flags = EXE_CLEAR;
  assert_eq!("-9999999999999999E+369", bid64_to_string(bid64, &mut flags));
  eqf(EXE_CLEAR, flags);
}
