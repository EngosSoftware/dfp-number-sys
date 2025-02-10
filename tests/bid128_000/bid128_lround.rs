use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  assert_eq!(-2, bid128_lround(d128("-1.75"), flags!()));
  assert_eq!(-2, bid128_lround(d128("-1.5"), flags!()));
  assert_eq!(-1, bid128_lround(d128("-1.25"), flags!()));
  assert_eq!(1, bid128_lround(d128("1.25"), flags!()));
  assert_eq!(2, bid128_lround(d128("1.5"), flags!()));
  assert_eq!(2, bid128_lround(d128("1.75"), flags!()));
}
