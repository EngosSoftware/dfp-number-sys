use super::*;

#[test]
fn _0001() {
  assert_eq!("[31C0000000000000]", format!("{:?}", d64("0")));
  assert!(bid64_is_zero(BID64_ZERO));
  eq("+0E+0", BID64_ZERO);
}

#[test]
fn _0002() {
  assert_eq!("[B1C0000000000000]", format!("{:?}", d64("-0")));
  assert!(bid64_is_zero(BID64_MINUS_ZERO));
  eq("-0E+0", BID64_MINUS_ZERO);
}

#[test]
fn _0003() {
  assert_eq!("[31C0000000000001]", format!("{:?}", d64("1")));
  eq("+1E+0", BID64_ONE);
}

#[test]
fn _0004() {
  assert_eq!("[B1C0000000000001]", format!("{:?}", d64("-1")));
  eq("-1E+0", BID64_MINUS_ONE);
}

#[test]
fn _0005() {
  assert_eq!("[31C0000000000002]", format!("{:?}", d64("2")));
  eq("+2E+0", BID64_TWO);
}

#[test]
fn _0006() {
  assert_eq!("[B1C0000000000002]", format!("{:?}", d64("-2")));
  eq("-2E+0", BID64_MINUS_TWO);
}

#[test]
fn _0007() {
  assert_eq!("[31A0000000000001]", format!("{:?}", d64("0.1")));
  eq("+1E-1", BID64_ONE_TENTH);
}

#[test]
fn _0008() {
  assert_eq!("[7800000000000000]", format!("{:?}", d64("Inf")));
  eq("+Inf", BID64_INF);
}

#[test]
fn _0009() {
  assert_eq!("[F800000000000000]", format!("{:?}", d64("-Inf")));
  eq("-Inf", BID64_MINUS_INF);
}

#[test]
fn _0010() {
  assert_eq!("[31C000003B9ACA00]", format!("{:?}", d64("1000000000")));
  eq("+1000000000E+0", BID64_BILLION);
}

#[test]
fn _0011() {
  assert_eq!("[B1C000003B9ACA00]", format!("{:?}", d64("-1000000000")));
  eq("-1000000000E+0", BID64_MINUS_BILLION);
}
