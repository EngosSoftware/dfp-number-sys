use super::*;

#[test]
fn _0001() {
  assert_eq!(-4_i64, bid128_llquantexp(d128("2.3456")));
}

#[test]
fn _0002() {
  assert_eq!(-7_i64, bid128_llquantexp(d128("122.4567000")));
}

#[test]
fn _0003() {
  assert_eq!(0_i64, bid128_llquantexp(d128("122000")));
}
