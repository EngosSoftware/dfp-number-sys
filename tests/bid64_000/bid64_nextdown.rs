use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-1000000000000001E-15", bid64_nextdown(d64("-1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999999999E-16", bid64_nextdown(d64("1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+5677999999999999E-15", bid64_nextdown(d64("5.678"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-5678000000000001E-15", bid64_nextdown(d64("-5.678"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid64_nextdown(d64("-Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9999999999999999E+369", bid64_nextdown(d64("+Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_nextdown(d64("+NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("-NaN", bid64_nextdown(d64("-NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_nextdown(d64("SNaN"), &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid64_nextdown(d64("QNaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
