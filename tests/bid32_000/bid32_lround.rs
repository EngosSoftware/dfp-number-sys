use super::*;

#[test]
fn _0001() {
  assert_eq!(-2, bid32_lround(d32("-1.75"), flags!()));
  assert_eq!(-2, bid32_lround(d32("-1.5"), flags!()));
  assert_eq!(-1, bid32_lround(d32("-1.25"), flags!()));
  assert_eq!(1, bid32_lround(d32("1.25"), flags!()));
  assert_eq!(2, bid32_lround(d32("1.5"), flags!()));
  assert_eq!(2, bid32_lround(d32("1.75"), flags!()));
}
