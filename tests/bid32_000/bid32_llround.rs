use super::*;

#[test]
fn _0001() {
  assert_eq!(-2_i64, bid32_llround(d32("-1.75"), flags!()));
  assert_eq!(-2_i64, bid32_llround(d32("-1.5"), flags!()));
  assert_eq!(-1_i64, bid32_llround(d32("-1.25"), flags!()));
  assert_eq!(1_i64, bid32_llround(d32("1.25"), flags!()));
  assert_eq!(2_i64, bid32_llround(d32("1.5"), flags!()));
  assert_eq!(2_i64, bid32_llround(d32("1.75"), flags!()));
}
