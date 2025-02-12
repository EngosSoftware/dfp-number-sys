use super::*;

#[test]
fn _0001() {
  assert_eq!(-2_i64, bid64_llround(d64("-1.75"), flags!()));
  assert_eq!(-2_i64, bid64_llround(d64("-1.5"), flags!()));
  assert_eq!(-1_i64, bid64_llround(d64("-1.25"), flags!()));
  assert_eq!(1_i64, bid64_llround(d64("1.25"), flags!()));
  assert_eq!(2_i64, bid64_llround(d64("1.5"), flags!()));
  assert_eq!(2_i64, bid64_llround(d64("1.75"), flags!()));
}
