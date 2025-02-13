use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid64_log(d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_log(d64("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999782785E-16", bid64_log(d64("2.7182818284"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+2302585092994046E-15", bid64_log(d64("10.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid64_log(d64("+Inf"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
