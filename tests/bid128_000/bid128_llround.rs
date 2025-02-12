use super::*;

#[test]
fn _0001() {
  assert_eq!(-2_i64, bid128_llround(d128("-1.75"), flags!()));
  assert_eq!(-2_i64, bid128_llround(d128("-1.5"), flags!()));
  assert_eq!(-1_i64, bid128_llround(d128("-1.25"), flags!()));
  assert_eq!(1_i64, bid128_llround(d128("1.25"), flags!()));
  assert_eq!(2_i64, bid128_llround(d128("1.5"), flags!()));
  assert_eq!(2_i64, bid128_llround(d128("1.75"), flags!()));
}
