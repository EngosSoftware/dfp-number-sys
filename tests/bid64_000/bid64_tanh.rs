use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid64_tanh(d64("0.0"), RND_NEAREST_EVEN, &mut flags);
  eq("+0E+0", result);
  assert!(bid64_is_zero(result));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7615941559557649E-16", bid64_tanh(d64("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-7615941559557649E-16", bid64_tanh(d64("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+9962720762238027E-16", bid64_tanh(d64("3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-9962720762238027E-16", bid64_tanh(d64("-3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9170257613966083E-16", bid64_tanh(d64("1.57"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-9170257613966083E-16", bid64_tanh(d64("-1.57"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
