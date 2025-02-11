use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  eq("-2E+0", bid128_nearbyint(d128("-1.75"), RM_NEAREST_AWAY, flags!()));
  eq("-2E+0", bid128_nearbyint(d128("-1.5"), RM_NEAREST_AWAY, flags!()));
  eq("-1E+0", bid128_nearbyint(d128("-1.25"), RM_NEAREST_AWAY, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.25"), RM_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("1.5"), RM_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("1.75"), RM_NEAREST_AWAY, flags!()));
}

#[test]
fn _0002() {
  eq("-3E+0", bid128_nearbyint(d128("-2.75"), RM_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid128_nearbyint(d128("-2.5"), RM_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid128_nearbyint(d128("-2.25"), RM_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("2.25"), RM_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("2.5"), RM_NEAREST_EVEN, flags!()));
  eq("+3E+0", bid128_nearbyint(d128("2.75"), RM_NEAREST_EVEN, flags!()));
}

#[test]
fn _0003() {
  eq("-1E+0", bid128_nearbyint(d128("-1.75"), RM_UPWARD, flags!()));
  eq("-1E+0", bid128_nearbyint(d128("-1.5"), RM_UPWARD, flags!()));
  eq("-1E+0", bid128_nearbyint(d128("-1.25"), RM_UPWARD, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("1.25"), RM_UPWARD, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("1.5"), RM_UPWARD, flags!()));
  eq("+2E+0", bid128_nearbyint(d128("1.75"), RM_UPWARD, flags!()));
}

#[test]
fn _0004() {
  eq("-2E+0", bid128_nearbyint(d128("-1.75"), RM_DOWNWARD, flags!()));
  eq("-2E+0", bid128_nearbyint(d128("-1.5"), RM_DOWNWARD, flags!()));
  eq("-2E+0", bid128_nearbyint(d128("-1.25"), RM_DOWNWARD, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.25"), RM_DOWNWARD, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.5"), RM_DOWNWARD, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.75"), RM_DOWNWARD, flags!()));
}

#[test]
fn _0005() {
  eq("-1E+0", bid128_nearbyint(d128("-1.75"), RM_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid128_nearbyint(d128("-1.5"), RM_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid128_nearbyint(d128("-1.25"), RM_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.25"), RM_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.5"), RM_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid128_nearbyint(d128("1.75"), RM_TOWARD_ZERO, flags!()));
}
