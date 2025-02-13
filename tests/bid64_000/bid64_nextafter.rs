use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-9999999999999999E-16", bid64_nextafter(d64("-1"), d64("1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999999999E-16", bid64_nextafter(d64("1"), d64("-1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+5678000000000001E-15", bid64_nextafter(d64("5.678"), d64("6.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-5678000000000001E-15", bid64_nextafter(d64("-5.678"), d64("-6.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-9999999999999999E+369", bid64_nextafter(d64("-Inf"), d64("+Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999999999E+369", bid64_nextafter(d64("+Inf"), d64("-Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_nextafter(d64("+NaN"), d64("1.0"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("-NaN", bid64_nextafter(d64("1.0"), d64("-NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
