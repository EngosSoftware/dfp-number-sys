use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_u16, bid32_to_uint16_ceil(d32("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_floor(d32("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_int(d32("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_rnint(d32("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_rninta(d32("1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_u16, bid32_to_uint16_xceil(d32("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_xfloor(d32("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_xint(d32("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_xrnint(d32("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_u16, bid32_to_uint16_xrninta(d32("1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}
