use super::*;

#[test]
fn _0001() {
  assert_eq!(-4_i64, bid32_llquantexp(d32("2.3456")));
}

#[test]
fn _0002() {
  assert_eq!(-11_i64, bid32_llquantexp(d32("122.4567E-7")));
}

#[test]
fn _0003() {
  assert_eq!(0_i64, bid32_llquantexp(d32("122000")));
}
