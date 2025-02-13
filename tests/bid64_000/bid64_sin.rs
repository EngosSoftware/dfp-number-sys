use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_sin(d64("0.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+8414709848078965E-16", bid64_sin(d64("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-8414709848078965E-16", bid64_sin(d64("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-4102067615373566E-25", bid64_sin(d64("3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+1000000000000000E-15", bid64_sin(d64("1.570796327"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
