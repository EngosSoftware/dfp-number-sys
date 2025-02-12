use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f64, bid128_to_binary64(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f64, bid128_to_binary64(d128("-0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert!(f64::is_infinite(bid128_to_binary64(
    d128("+9999999999999999999999999999999999E+6111"),
    RND_NEAREST_EVEN,
    &mut flags
  )));
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  assert_eq!(f64::MAX, bid128_to_binary64(d128("1.7976931348623157e308"), RND_NEAREST_AWAY, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert!(f64::is_infinite(bid128_to_binary64(
    d128("-9999999999999999999999999999999999E+6111"),
    RND_NEAREST_EVEN,
    &mut flags
  )));
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  assert_eq!(f64::MIN, bid128_to_binary64(d128("-1.7976931348623157e308"), RND_NEAREST_AWAY, &mut flags));
  eqf(EXE_INEXACT, flags);
}
