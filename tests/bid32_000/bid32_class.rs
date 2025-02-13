use super::*;

#[test]
fn _0001() {
  assert_eq!(Class::SignalingNaN, bid32_class(d32("SNaN")));
  assert_eq!(0, bid32_class(d32("SNaN")) as u32);
}

#[test]
fn _0002() {
  assert_eq!(Class::QuietNaN, bid32_class(d32("QNaN")));
  assert_eq!(1, bid32_class(d32("QNaN")) as u32);
}

#[test]
fn _0003() {
  assert_eq!(Class::NegativeInfinity, bid32_class(d32("-Inf")));
  assert_eq!(2, bid32_class(d32("-Inf")) as u32);
}

#[test]
fn _0004() {
  assert_eq!(Class::NegativeNormal, bid32_class(d32("-1")));
  assert_eq!(3, bid32_class(d32("-1")) as u32);
}

#[test]
fn _0005() {
  assert_eq!(Class::NegativeSubnormal, bid32_class(bid32_negative_subnormal()));
  assert_eq!(4, bid32_class(bid32_negative_subnormal()) as u32);
}

#[test]
fn _0006() {
  assert_eq!(Class::NegativeZero, bid32_class(d32("-0")));
  assert_eq!(5, bid32_class(d32("-0")) as u32);
}

#[test]
fn _0007() {
  assert_eq!(Class::PositiveZero, bid32_class(d32("+0")));
  assert_eq!(6, bid32_class(d32("+0")) as u32);
}

#[test]
fn _0008() {
  assert_eq!(Class::PositiveSubnormal, bid32_class(bid32_positive_subnormal()));
  assert_eq!(7, bid32_class(bid32_positive_subnormal()) as u32);
}

#[test]
fn _0009() {
  assert_eq!(Class::PositiveNormal, bid32_class(d32("+1")));
  assert_eq!(8, bid32_class(d32("+1")) as u32);
}

#[test]
fn _0010() {
  assert_eq!(Class::PositiveInfinity, bid32_class(d32("+Inf")));
  assert_eq!(9, bid32_class(d32("+Inf")) as u32);
}

#[test]
#[should_panic(expected = "internal error: entered unreachable code")]
fn _0011() {
  let _: Class = 10.into();
}
