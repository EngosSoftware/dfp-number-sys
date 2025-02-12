#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  assert_eq!(true, bid128_total_order_mag(d128("-1"), d128("2")));
}
