use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f32, bid32_to_binary32(d32("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f32, bid32_to_binary32(d32("-0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert!(f32::is_infinite(bid32_to_binary32(d32(&format!("{:+e}", f32::MAX)), RND_NEAREST_AWAY, &mut flags)));
  eqf(EXE_INEXACT | EXE_OVERFLOW, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(f32::MAX, bid32_to_binary32(d32(&format!("{:+e}", f32::MAX)), RND_TOWARD_ZERO, &mut flags));
  eqf(EXE_INEXACT | EXE_OVERFLOW, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  assert_eq!(f32::MIN, bid32_to_binary32(d32(&format!("{:+e}", f32::MIN)), RND_TOWARD_ZERO, &mut flags));
  eqf(EXE_INEXACT | EXE_OVERFLOW, flags);
}
