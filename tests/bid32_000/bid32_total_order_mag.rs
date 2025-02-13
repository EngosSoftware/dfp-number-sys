#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  assert_eq!(true, bid32_total_order_mag(d32("-1"), d32("2")));
}
