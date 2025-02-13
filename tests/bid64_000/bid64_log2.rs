use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid64_log2(d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_log2(d64("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+3E+0", bid64_log2(d64("8.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid64_log2(d64("2.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+4559491813205038E-15", bid64_log2(d64("23.58"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid64_log2(d64("+Inf"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_log2(d64("-2.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INVALID, flags);
}
