use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid64_tan(d64("0.0"), RND_NEAREST_EVEN, &mut flags);
  eq("+0E+0", result);
  assert!(bid64_is_zero(result));
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1557407724654902E-15", bid64_tan(d64("1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-1557407724654902E-15", bid64_tan(d64("-1.0"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+4102067615373566E-25", bid64_tan(d64("3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-4102067615373566E-25", bid64_tan(d64("-3.141592654"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9262049631670410E-14", bid64_tan(d64("1.56"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("-1086492036048439E-13", bid64_tan(d64("1.58"), RND_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, EXE_INEXACT);
}
