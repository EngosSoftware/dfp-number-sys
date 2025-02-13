//! # Sanity tests
//!
//! ```text
//! 0 0 0
//! │ │ │
//! │ │ └─ status flags passed as an argument
//! │ └─── rounding mode passed as an argument
//! └───── result returned by value
//! /// ```

use super::*;

#[test]
fn test_bid128_round_integral_nearest_away() {
  eq("-2E+0", bid128_round_integral_nearest_away(d128("-2.0"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.9"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.8"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.7"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.6"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_away(d128("-1.5"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_away(d128("-1.4"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_away(d128("-1.3"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_away(d128("-1.2"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_away(d128("-1.1"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_away(d128("-1.0"), flags!()));
  eq("+1E+0", bid128_round_integral_nearest_away(d128("1.0"), flags!()));
  eq("+1E+0", bid128_round_integral_nearest_away(d128("1.1"), flags!()));
  eq("+1E+0", bid128_round_integral_nearest_away(d128("1.2"), flags!()));
  eq("+1E+0", bid128_round_integral_nearest_away(d128("1.3"), flags!()));
  eq("+1E+0", bid128_round_integral_nearest_away(d128("1.4"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("1.5"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("1.6"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("1.6"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("1.7"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("1.8"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("1.9"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_away(d128("2.0"), flags!()));
}

#[test]
fn test_bid128_round_integral_exact_nearest_away() {
  eq("-2E+0", bid128_round_integral_exact(d128("-1.75"), RND_NEAREST_AWAY, flags!()));
  eq("-2E+0", bid128_round_integral_exact(d128("-1.5"), RND_NEAREST_AWAY, flags!()));
  eq("-1E+0", bid128_round_integral_exact(d128("-1.25"), RND_NEAREST_AWAY, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.25"), RND_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("1.5"), RND_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("1.75"), RND_NEAREST_AWAY, flags!()));
}

#[test]
fn test_bid128_round_integral_nearest_even() {
  eq("-2E+0", bid128_round_integral_nearest_even(d128("-2.0"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_even(d128("-1.9"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_even(d128("-1.8"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_even(d128("-1.7"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_even(d128("-1.6"), flags!()));
  eq("-2E+0", bid128_round_integral_nearest_even(d128("-1.5"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_even(d128("-1.4"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_even(d128("-1.3"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_even(d128("-1.2"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_even(d128("-1.1"), flags!()));
  eq("-1E+0", bid128_round_integral_nearest_even(d128("-1.0"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_even(d128("2.25"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_even(d128("2.5"), flags!()));
  eq("+3E+0", bid128_round_integral_nearest_even(d128("2.75"), flags!()));

  eq("+0E+0", bid128_round_integral_nearest_even(d128("0.5"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_even(d128("1.5"), flags!()));
  eq("+2E+0", bid128_round_integral_nearest_even(d128("2.5"), flags!()));
  eq("+4E+0", bid128_round_integral_nearest_even(d128("3.5"), flags!()));
  eq("+4E+0", bid128_round_integral_nearest_even(d128("4.5"), flags!()));
}

#[test]
fn test_bid128_round_integral_exact_nearest_even() {
  eq("-3E+0", bid128_round_integral_exact(d128("-2.75"), RND_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid128_round_integral_exact(d128("-2.5"), RND_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid128_round_integral_exact(d128("-2.25"), RND_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("2.25"), RND_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("2.5"), RND_NEAREST_EVEN, flags!()));
  eq("+3E+0", bid128_round_integral_exact(d128("2.75"), RND_NEAREST_EVEN, flags!()));
}

#[test]
fn test_bid128_round_integral_positive() {
  eq("-1E+0", bid128_round_integral_positive(d128("-1.75"), flags!()));
  eq("-1E+0", bid128_round_integral_positive(d128("-1.5"), flags!()));
  eq("-1E+0", bid128_round_integral_positive(d128("-1.25"), flags!()));
  eq("+2E+0", bid128_round_integral_positive(d128("1.25"), flags!()));
  eq("+2E+0", bid128_round_integral_positive(d128("1.5"), flags!()));
  eq("+2E+0", bid128_round_integral_positive(d128("1.75"), flags!()));
}

#[test]
fn test_bid128_round_integral_exact_upward() {
  eq("-1E+0", bid128_round_integral_exact(d128("-1.75"), RND_UPWARD, flags!()));
  eq("-1E+0", bid128_round_integral_exact(d128("-1.5"), RND_UPWARD, flags!()));
  eq("-1E+0", bid128_round_integral_exact(d128("-1.25"), RND_UPWARD, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("1.25"), RND_UPWARD, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("1.5"), RND_UPWARD, flags!()));
  eq("+2E+0", bid128_round_integral_exact(d128("1.75"), RND_UPWARD, flags!()));
}

#[test]
fn test_bid128_round_integral_negative() {
  eq("-2E+0", bid128_round_integral_negative(d128("-1.75"), flags!()));
  eq("-2E+0", bid128_round_integral_negative(d128("-1.5"), flags!()));
  eq("-2E+0", bid128_round_integral_negative(d128("-1.25"), flags!()));
  eq("+1E+0", bid128_round_integral_negative(d128("1.25"), flags!()));
  eq("+1E+0", bid128_round_integral_negative(d128("1.5"), flags!()));
  eq("+1E+0", bid128_round_integral_negative(d128("1.75"), flags!()));
}

#[test]
fn test_bid128_round_integral_downward() {
  eq("-2E+0", bid128_round_integral_exact(d128("-1.75"), RND_DOWNWARD, flags!()));
  eq("-2E+0", bid128_round_integral_exact(d128("-1.5"), RND_DOWNWARD, flags!()));
  eq("-2E+0", bid128_round_integral_exact(d128("-1.25"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.25"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.5"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.75"), RND_DOWNWARD, flags!()));
}

#[test]
fn test_bid128_round_integral_zero() {
  eq("-1E+0", bid128_round_integral_zero(d128("-1.75"), flags!()));
  eq("-1E+0", bid128_round_integral_zero(d128("-1.5"), flags!()));
  eq("-1E+0", bid128_round_integral_zero(d128("-1.25"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.0"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.1"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.2"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.3"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.4"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.5"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.6"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.7"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.8"), flags!()));
  eq("+1E+0", bid128_round_integral_zero(d128("1.9"), flags!()));
  eq("+2E+0", bid128_round_integral_zero(d128("2.0"), flags!()));
}

#[test]
fn test_bid128_round_integral_toward_zero() {
  eq("-1E+0", bid128_round_integral_exact(d128("-1.75"), RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid128_round_integral_exact(d128("-1.5"), RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid128_round_integral_exact(d128("-1.25"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.25"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.5"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid128_round_integral_exact(d128("1.75"), RND_TOWARD_ZERO, flags!()));
}

#[test]
fn test_bid128_scalbn_0001() {
  let x = bid128_scalbn(bid128_from_int64(2356789100), -9);
  eq("+2356789100E-9", x);
}

#[test]
fn test_bid128_scalbn_0002() {
  let x = bid128_scalbn(bid128_from_int64(2356789100), -9);
  let y = bid128_scalbn(x, 2);
  eq("+2356789100E-7", y);
}

#[test]
fn test_bid128_scalbn_0003() {
  let x = bid128_scalbn(bid128_from_int64(1), 6144);
  eq("+1000000000000000000000000000000000E+6111", x);
}

#[test]
fn test_bid128_scalbn_0004() {
  let x = bid128_scalbn(bid128_from_int64(1), -6176);
  eq("+1E-6176", x);
}

#[test]
fn test_bid128_scalbln_0001() {
  let x = bid128_scalbln(bid128_from_int64(2356789100), -9);
  eq("+2356789100E-9", x);
}

#[test]
fn test_bid128_scalbln_0002() {
  let x = bid128_scalbln(bid128_from_int64(2356789100), -9);
  let y = bid128_scalbln(x, 2);
  eq("+2356789100E-7", y);
}

#[test]
fn test_bid128_scalbln_0003() {
  let x = bid128_scalbln(bid128_from_int64(1), 6144);
  eq("+1000000000000000000000000000000000E+6111", x);
}

#[test]
fn test_bid128_scalbln_0004() {
  let x = bid128_scalbln(bid128_from_int64(1), -6176);
  eq("+1E-6176", x);
}

#[test]
fn test_bid128_sqrt_0001() {
  eq("+1414213562373095048801688724209698E-33", bid128_sqrt(d128("2"), RND_NEAREST_EVEN, flags!()));
}

#[test]
fn test_bid128_sqrt_0002() {
  let x = bid128_infinite();
  let y = bid128_sqrt(x, RND_NEAREST_EVEN, flags!());
  assert!(!bid128_is_finite(y));
}

#[test]
fn test_bid128_to_int32_int() {
  assert_eq!(0, bid128_to_int32_int(d128("0"), flags!()));
  assert_eq!(0, bid128_to_int32_int(d128("0.12"), flags!()));
  assert_eq!(0, bid128_to_int32_int(d128("0.99"), flags!()));
  assert_eq!(0, bid128_to_int32_int(d128("-0.12"), flags!()));
  assert_eq!(0, bid128_to_int32_int(d128("-0.99"), flags!()));
  assert_eq!(2147483647, bid128_to_int32_int(d128("2147483647.999"), flags!()));
  assert_eq!(-2147483648, bid128_to_int32_int(d128("-2147483648.999"), flags!()));
  let mut flags = EXE_CLEAR;
  assert_eq!(-2147483648, bid128_to_int32_int(d128("21474836483453459382.7423947"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
  let mut flags = EXE_CLEAR;
  assert_eq!(-2147483648, bid128_to_int32_int(d128("-21474836483453459.3827423947"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
}

#[test]
fn test_bid128_to_uint32_int() {
  assert_eq!(0, bid128_to_uint32_int(d128("0"), flags!()));
  assert_eq!(0, bid128_to_uint32_int(d128("0.12"), flags!()));
  assert_eq!(0, bid128_to_uint32_int(d128("0.99"), flags!()));
  assert_eq!(4294967295, bid128_to_uint32_int(d128("4294967295.999"), flags!()));
  let mut flags = EXE_CLEAR;
  assert_eq!(2147483648, bid128_to_uint32_int(d128("214748364834534593829384"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
  let mut flags = EXE_CLEAR;
  assert_eq!(2147483648, bid128_to_uint32_int(d128("-21474836483453459.3827423947"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
}

#[test]
fn test_bid128_to_int64_int() {
  assert_eq!(0, bid128_to_int64_int(d128("0"), flags!()));
  assert_eq!(0, bid128_to_int64_int(d128("0.12"), flags!()));
  assert_eq!(0, bid128_to_int64_int(d128("0.99"), flags!()));
  assert_eq!(0, bid128_to_int64_int(d128("-0.12"), flags!()));
  assert_eq!(0, bid128_to_int64_int(d128("-0.99"), flags!()));
  assert_eq!(9223372036854775807, bid128_to_int64_int(d128("9223372036854775807.999"), flags!()));
  assert_eq!(-9223372036854775808, bid128_to_int64_int(d128("-9223372036854775808.999"), flags!()));
  let mut flags = EXE_CLEAR;
  assert_eq!(-9223372036854775808, bid128_to_int64_int(d128("921474836483453459382349857.74239"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
  let mut flags = EXE_CLEAR;
  assert_eq!(-9223372036854775808, bid128_to_int64_int(d128("-9214748364834534599487453534.3827"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
}

#[test]
fn test_bid128_to_uint64_int() {
  assert_eq!(0, bid128_to_uint64_int(d128("0"), flags!()));
  assert_eq!(0, bid128_to_uint64_int(d128("0.12"), flags!()));
  assert_eq!(0, bid128_to_uint64_int(d128("0.99"), flags!()));
  assert_eq!(18446744073709551615, bid128_to_uint64_int(d128("18446744073709551615.999"), flags!()));
  let mut flags = EXE_CLEAR;
  assert_eq!(9223372036854775808, bid128_to_uint64_int(d128("3498375214748364834534593829384"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
  let mut flags = EXE_CLEAR;
  assert_eq!(9223372036854775808, bid128_to_uint64_int(d128("-21474836483453459.3827423947"), &mut flags));
  assert_eq!(EXE_INVALID, flags);
}

#[test]
fn test_bid128_infinite() {
  let x = bid128_infinite();
  assert!(bid128_is_infinite(x));
  assert!(!bid128_is_finite(x));
}
