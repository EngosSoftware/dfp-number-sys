use super::*;

#[test]
fn _0001() {
  assert_eq!(-4_i32, bid32_quantexp(d32("2.3456")));
}

#[test]
fn _0002() {
  assert_eq!(-11_i32, bid32_quantexp(d32("123.456E-8")));
}

#[test]
fn _0003() {
  assert_eq!(0_i32, bid32_quantexp(d32("122000")));
}
