use super::*;

#[test]
fn _0001() {
  assert_eq!(Class::SignalingNaN, bid128_class(d128("SNaN")));
  assert_eq!(0, bid128_class(d128("SNaN")) as u32);
}

#[test]
fn _0002() {
  assert_eq!(Class::QuietNaN, bid128_class(d128("QNaN")));
  assert_eq!(1, bid128_class(d128("QNaN")) as u32);
}

#[test]
fn _0003() {
  assert_eq!(Class::NegativeInfinity, bid128_class(d128("-Inf")));
  assert_eq!(2, bid128_class(d128("-Inf")) as u32);
}

#[test]
fn _0004() {
  assert_eq!(Class::NegativeNormal, bid128_class(d128("-1")));
  assert_eq!(3, bid128_class(d128("-1")) as u32);
}

#[test]
fn _0005() {
  assert_eq!(Class::NegativeSubnormal, bid128_class(bid128_negative_subnormal()));
  assert_eq!(4, bid128_class(bid128_negative_subnormal()) as u32);
}

#[test]
fn _0006() {
  assert_eq!(Class::NegativeZero, bid128_class(d128("-0")));
  assert_eq!(5, bid128_class(d128("-0")) as u32);
}

#[test]
fn _0007() {
  assert_eq!(Class::PositiveZero, bid128_class(d128("+0")));
  assert_eq!(6, bid128_class(d128("+0")) as u32);
}

#[test]
fn _0008() {
  assert_eq!(Class::PositiveSubnormal, bid128_class(bid128_positive_subnormal()));
  assert_eq!(7, bid128_class(bid128_positive_subnormal()) as u32);
}

#[test]
fn _0009() {
  assert_eq!(Class::PositiveNormal, bid128_class(d128("+1")));
  assert_eq!(8, bid128_class(d128("+1")) as u32);
}

#[test]
fn _0010() {
  assert_eq!(Class::PositiveInfinity, bid128_class(d128("+Inf")));
  assert_eq!(9, bid128_class(d128("+Inf")) as u32);
}

#[test]
#[should_panic(expected = "internal error: entered unreachable code")]
fn _0011() {
  let _: Class = 10.into();
}
