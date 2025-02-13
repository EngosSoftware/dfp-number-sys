use super::*;

#[test]
fn _0001() {
  eq("-12345E-4", bid32_negate(d32("+1.2345")));
}

#[test]
fn _0002() {
  eq("+12345E-4", bid32_negate(d32("-1.2345")));
}

#[test]
fn _0003() {
  eq("-0E+0", bid32_negate(d32("+0")));
}

#[test]
fn _0004() {
  eq("+0E+0", bid32_negate(d32("-0")));
}
