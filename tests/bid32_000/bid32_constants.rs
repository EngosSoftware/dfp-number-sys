use super::*;

#[test]
fn _0001() {
  assert_eq!("[32800000]", format!("{:?}", d32("0")));
  assert!(bid32_is_zero(BID32_ZERO));
  eq("+0E+0", BID32_ZERO);
}

#[test]
fn _0002() {
  assert_eq!("[B2800000]", format!("{:?}", d32("-0")));
  assert!(bid32_is_zero(BID32_MINUS_ZERO));
  eq("-0E+0", BID32_MINUS_ZERO);
}

#[test]
fn _0003() {
  assert_eq!("[32800001]", format!("{:?}", d32("1")));
  eq("+1E+0", BID32_ONE);
}

#[test]
fn _0004() {
  assert_eq!("[B2800001]", format!("{:?}", d32("-1")));
  eq("-1E+0", BID32_MINUS_ONE);
}

#[test]
fn _0005() {
  assert_eq!("[32800002]", format!("{:?}", d32("2")));
  eq("+2E+0", BID32_TWO);
}

#[test]
fn _0006() {
  assert_eq!("[B2800002]", format!("{:?}", d32("-2")));
  eq("-2E+0", BID32_MINUS_TWO);
}

#[test]
fn _0007() {
  assert_eq!("[32000001]", format!("{:?}", d32("0.1")));
  eq("+1E-1", BID32_ONE_TENTH);
}

#[test]
fn _0008() {
  assert_eq!("[78000000]", format!("{:?}", d32("Inf")));
  eq("+Inf", BID32_INF);
}

#[test]
fn _0009() {
  assert_eq!("[F8000000]", format!("{:?}", d32("-Inf")));
  eq("-Inf", BID32_MINUS_INF);
}

#[test]
fn _0010() {
  assert_eq!("[340F4240]", format!("{:?}", d32("1000000000")));
  eq("+1000000E+3", BID32_BILLION);
}

#[test]
fn _0011() {
  assert_eq!("[B40F4240]", format!("{:?}", d32("-1000000000")));
  eq("-1000000E+3", BID32_MINUS_BILLION);
}

#[test]
fn _0012() {
  assert_eq!("[F7F8967F]", format!("{:?}", d32("-9999999E+90")));
  eq("-9999999E+90", BID32_MIN);
}

#[test]
fn _0013() {
  assert_eq!("[77F8967F]", format!("{:?}", d32("+9999999E+90")));
  eq("+9999999E+90", BID32_MAX);
}
