use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid128_tanh(d128("0.0"), RND_NEAREST_EVEN, &mut flags);
  eq("+0E-1", result);
  assert!(bid128_is_zero(result));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+7615941559557648881194582826047936E-34", bid128_tanh(d128("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-7615941559557648881194582826047936E-34", bid128_tanh(d128("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+9962720762238026825310418977951963E-34", bid128_tanh(d128("3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-9962720762238026825310418977951963E-34", bid128_tanh(d128("-3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9170257613966082987846433611735103E-34", bid128_tanh(d128("1.57"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-9170257613966082987846433611735103E-34", bid128_tanh(d128("-1.57"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
