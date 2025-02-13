use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_ldexp(d64("0"), 0, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E-1", bid64_ldexp(d64("0"), -1, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+0E+1", bid64_ldexp(d64("0"), 1, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+15E-1", bid64_ldexp(d64("1.5"), 0, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+15E+0", bid64_ldexp(d64("1.5"), 1, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+15E+1", bid64_ldexp(d64("1.5"), 2, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+15E+7", bid64_ldexp(d64("1.5"), 8, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("+15E-9", bid64_ldexp(d64("1.5"), -8, RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
