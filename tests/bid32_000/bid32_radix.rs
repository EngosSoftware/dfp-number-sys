use super::*;

#[test]
fn _0001() {
  assert_eq!(10, bid32_radix(d32("2")));
}

#[test]
fn _0002() {
  assert_eq!(10, bid32_radix(d32("2.498537")));
}

#[test]
fn _0003() {
  assert_eq!(10, bid32_radix(d32("Inf")));
}
