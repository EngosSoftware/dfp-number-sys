use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_i16, bid64_to_int16_ceil(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_ceil(d64("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_floor(d64("1.2"), &mut flags));
  assert_eq!(-2_i16, bid64_to_int16_floor(d64("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_int(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_int(d64("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_rnint(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_rnint(d64("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_rninta(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_rninta(d64("-1.2"), &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  assert_eq!(2_i16, bid64_to_int16_xceil(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_xceil(d64("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_xfloor(d64("1.2"), &mut flags));
  assert_eq!(-2_i16, bid64_to_int16_xfloor(d64("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_xint(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_xint(d64("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_xrnint(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_xrnint(d64("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0010() {
  let mut flags = EXE_CLEAR;
  assert_eq!(1_i16, bid64_to_int16_xrninta(d64("1.2"), &mut flags));
  assert_eq!(-1_i16, bid64_to_int16_xrninta(d64("-1.2"), &mut flags));
  eqf(EXE_INEXACT, flags);
}
