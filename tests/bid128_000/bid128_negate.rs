use super::*;

#[test]
fn _0001() {
  eq("-12345E-4", bid128_negate(d128("+1.2345")));
}

#[test]
fn _0002() {
  eq("+12345E-4", bid128_negate(d128("-1.2345")));
}

#[test]
fn _0003() {
  eq("-0E+0", bid128_negate(d128("+0")));
}

#[test]
fn _0004() {
  eq("+0E+0", bid128_negate(d128("-0")));
}
