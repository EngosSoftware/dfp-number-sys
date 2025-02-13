use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_sinh(d64("0.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1175201193643801E-15", bid64_sinh(d64("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-1175201193643801E-15", bid64_sinh(d64("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1154873936201285E-14", bid64_sinh(d64("3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-1154873936201285E-14", bid64_sinh(d64("-3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+2301298902821936E-15", bid64_sinh(d64("1.570796327"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-2301298902821936E-15", bid64_sinh(d64("-1.570796327"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
