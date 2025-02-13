use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+2E+0", bid64_fmod(d64("5"), d64("3"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+33E-1", bid64_fmod(d64("3.3"), d64("5.4"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-25E-1", bid64_fmod(d64("-5.2"), d64("2.7"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_fmod(d64("1.0"), d64("0"), &mut flags));
  eqf(EXE_INVALID, flags);
}
