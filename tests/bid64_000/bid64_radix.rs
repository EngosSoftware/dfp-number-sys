use super::*;

#[test]
fn _0001() {
  assert_eq!(10, bid64_radix(d64("2")));
}

#[test]
fn _0002() {
  assert_eq!(10, bid64_radix(d64("2.498537")));
}

#[test]
fn _0003() {
  assert_eq!(10, bid64_radix(d64("Inf")));
}
