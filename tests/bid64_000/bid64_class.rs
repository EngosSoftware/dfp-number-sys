use super::*;

#[test]
fn _0001() {
  assert_eq!(Class::SignalingNaN, bid64_class(d64("SNaN")));
  assert_eq!(0, bid64_class(d64("SNaN")) as u32);
}

#[test]
fn _0002() {
  assert_eq!(Class::QuietNaN, bid64_class(d64("QNaN")));
  assert_eq!(1, bid64_class(d64("QNaN")) as u32);
}

#[test]
fn _0003() {
  assert_eq!(Class::NegativeInfinity, bid64_class(d64("-Inf")));
  assert_eq!(2, bid64_class(d64("-Inf")) as u32);
}

#[test]
fn _0004() {
  assert_eq!(Class::NegativeNormal, bid64_class(d64("-1")));
  assert_eq!(3, bid64_class(d64("-1")) as u32);
}

#[test]
fn _0005() {
  assert_eq!(Class::NegativeSubnormal, bid64_class(bid64_negative_subnormal()));
  assert_eq!(4, bid64_class(bid64_negative_subnormal()) as u32);
}

#[test]
fn _0006() {
  assert_eq!(Class::NegativeZero, bid64_class(d64("-0")));
  assert_eq!(5, bid64_class(d64("-0")) as u32);
}

#[test]
fn _0007() {
  assert_eq!(Class::PositiveZero, bid64_class(d64("+0")));
  assert_eq!(6, bid64_class(d64("+0")) as u32);
}

#[test]
fn _0008() {
  assert_eq!(Class::PositiveSubnormal, bid64_class(bid64_positive_subnormal()));
  assert_eq!(7, bid64_class(bid64_positive_subnormal()) as u32);
}

#[test]
fn _0009() {
  assert_eq!(Class::PositiveNormal, bid64_class(d64("+1")));
  assert_eq!(8, bid64_class(d64("+1")) as u32);
}

#[test]
fn _0010() {
  assert_eq!(Class::PositiveInfinity, bid64_class(d64("+Inf")));
  assert_eq!(9, bid64_class(d64("+Inf")) as u32);
}

#[test]
#[should_panic(expected = "internal error: entered unreachable code")]
fn _0011() {
  let _: Class = 10.into();
}
