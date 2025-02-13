use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-1000001E-6", bid32_nextdown(d32("-1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+9999999E-7", bid32_nextdown(d32("1"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+5677999E-6", bid32_nextdown(d32("5.678"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("-5678001E-6", bid32_nextdown(d32("-5.678"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid32_nextdown(d32("-Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+9999999E+90", bid32_nextdown(d32("+Inf"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_nextdown(d32("+NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("-NaN", bid32_nextdown(d32("-NaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_nextdown(d32("SNaN"), &mut flags));
  eqf(EXE_INVALID, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  eq("+NaN", bid32_nextdown(d32("QNaN"), &mut flags));
  eqf(EXE_CLEAR, flags);
}
