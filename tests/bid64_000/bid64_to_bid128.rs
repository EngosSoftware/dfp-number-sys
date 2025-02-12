use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let value = bid64_to_bid128(d64("0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("+0E+0", bid128_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  let value = bid64_to_bid128(d64("-0"), RND_NEAREST_EVEN, &mut flags);
  eqf(EXE_CLEAR, flags);
  assert_eq!("-0E+0", bid128_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  let value = bid64_to_bid128(d64("+9999999999999999E+308"), RND_TOWARD_ZERO, &mut flags);
  eqf(EXE_CLEAR, flags);
  flags = EXE_CLEAR;
  assert_eq!("+9999999999999999E+308", bid128_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  let value = bid64_to_bid128(d64("-9999999999999999E+308"), RND_TOWARD_ZERO, &mut flags);
  eqf(EXE_CLEAR, flags);
  flags = EXE_CLEAR;
  assert_eq!("-9999999999999999E+308", bid128_to_string(value, &mut flags));
  eqf(EXE_CLEAR, flags);
}
