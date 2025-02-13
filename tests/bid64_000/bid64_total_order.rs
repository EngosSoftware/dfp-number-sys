#![allow(clippy::bool_assert_comparison)]

use super::*;

#[test]
fn _0001() {
  assert_eq!(true, bid64_total_order(d64("1"), d64("2")));
}
