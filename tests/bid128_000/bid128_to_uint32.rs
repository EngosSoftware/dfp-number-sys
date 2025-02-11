use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  assert_eq!(2_u32, bid128_to_uint32_ceil(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_floor(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_int(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_rnint(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_rninta(d128("1.2"), &mut flags));
  eqf(FB_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  assert_eq!(2_u32, bid128_to_uint32_xceil(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_xfloor(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0008() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_xint(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0009() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_xrnint(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}

#[test]
fn _0010() {
  let mut flags = FB_CLEAR;
  assert_eq!(1_u32, bid128_to_uint32_xrninta(d128("1.2"), &mut flags));
  eqf(FB_INEXACT, flags);
}
