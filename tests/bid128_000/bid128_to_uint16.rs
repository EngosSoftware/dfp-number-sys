use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  assert_eq!(2_u16, bid128_to_uint16_ceil(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_floor(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_int(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_rnint(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_rninta(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  assert_eq!(2_u16, bid128_to_uint16_xceil(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_xfloor(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0008() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_xint(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0009() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_xrnint(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0010() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u16, bid128_to_uint16_xrninta(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}
