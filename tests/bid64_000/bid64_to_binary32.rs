use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f32, bid64_to_binary32(d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f32, bid64_to_binary32(d64("-0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert!(f32::is_infinite(bid64_to_binary32(d64("+9999999999999999E+308"), RND_NEAREST_EVEN, &mut flags)));
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(f32::MAX, bid64_to_binary32(d64("3.40282347E+38"), RND_NEAREST_AWAY, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert!(f32::is_infinite(bid64_to_binary32(d64("-9999999999999999E+308"), RND_NEAREST_EVEN, &mut flags)));
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  assert_eq!(f32::MIN, bid64_to_binary32(d64("-3.40282347E+38"), RND_NEAREST_AWAY, &mut flags));
  eqf(EXE_INEXACT, flags);
}
