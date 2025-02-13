use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f64, bid32_to_binary64(d32("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f64, bid32_to_binary64(d32("-0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert_eq!(3.402824e38_f64, bid32_to_binary64(d32(&format!("{:+e}", f32::MAX)), RND_TOWARD_ZERO, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(-3.402824e38_f64, bid32_to_binary64(d32(&format!("{:+e}", f32::MIN)), RND_TOWARD_ZERO, &mut flags));
  eqf(EXE_INEXACT, flags);
}
