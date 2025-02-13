use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let value = bid32_to_bid64(d32("0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("+0E+0", bid64_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let value = bid32_to_bid64(d32("-0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("-0E+0", bid64_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let value = bid32_to_bid64(BID32_MAX, RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("+9999999E+90", bid64_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  let value = bid32_to_bid64(BID32_MIN, RND_TOWARD_ZERO, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("-9999999E+90", bid64_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}
