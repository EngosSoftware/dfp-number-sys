use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid64_logb(d64("0"), &mut flags));
  eqf(EXE_ZERO_DIVIDE, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid64_logb(d64("1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+1E+0", bid64_logb(d64("10.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid64_logb(d64("256.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-4E+0", bid64_logb(d64("0.0001"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid64_logb(d64("+Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
