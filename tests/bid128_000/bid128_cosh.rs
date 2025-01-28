use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+1000000000000000000000000000000000E-33", bid128_cosh(d128("0.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1543080634815243778477905620757062E-33", bid128_cosh(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("+1543080634815243778477905620757062E-33", bid128_cosh(d128("-1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+1159195328025889160030675296169936E-32", bid128_cosh(d128("3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+1159195328025889160030675296169936E-32", bid128_cosh(d128("-3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+2509178479130060967085247471088830E-33", bid128_cosh(d128("1.570796327"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  eq("+2509178479130060967085247471088830E-33", bid128_cosh(d128("-1.570796327"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}
