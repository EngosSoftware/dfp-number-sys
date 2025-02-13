use super::*;

#[test]
fn _0001() {
  eq("-2E+0", bid32_nearbyint(d32("-1.75"), RND_NEAREST_AWAY, flags!()));
  eq("-2E+0", bid32_nearbyint(d32("-1.5"), RND_NEAREST_AWAY, flags!()));
  eq("-1E+0", bid32_nearbyint(d32("-1.25"), RND_NEAREST_AWAY, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.25"), RND_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("1.5"), RND_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("1.75"), RND_NEAREST_AWAY, flags!()));
}

#[test]
fn _0002() {
  eq("-3E+0", bid32_nearbyint(d32("-2.75"), RND_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid32_nearbyint(d32("-2.5"), RND_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid32_nearbyint(d32("-2.25"), RND_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("2.25"), RND_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("2.5"), RND_NEAREST_EVEN, flags!()));
  eq("+3E+0", bid32_nearbyint(d32("2.75"), RND_NEAREST_EVEN, flags!()));
}

#[test]
fn _0003() {
  eq("-1E+0", bid32_nearbyint(d32("-1.75"), RND_UPWARD, flags!()));
  eq("-1E+0", bid32_nearbyint(d32("-1.5"), RND_UPWARD, flags!()));
  eq("-1E+0", bid32_nearbyint(d32("-1.25"), RND_UPWARD, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("1.25"), RND_UPWARD, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("1.5"), RND_UPWARD, flags!()));
  eq("+2E+0", bid32_nearbyint(d32("1.75"), RND_UPWARD, flags!()));
}

#[test]
fn _0004() {
  eq("-2E+0", bid32_nearbyint(d32("-1.75"), RND_DOWNWARD, flags!()));
  eq("-2E+0", bid32_nearbyint(d32("-1.5"), RND_DOWNWARD, flags!()));
  eq("-2E+0", bid32_nearbyint(d32("-1.25"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.25"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.5"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.75"), RND_DOWNWARD, flags!()));
}

#[test]
fn _0005() {
  eq("-1E+0", bid32_nearbyint(d32("-1.75"), RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid32_nearbyint(d32("-1.5"), RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid32_nearbyint(d32("-1.25"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.25"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.5"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_nearbyint(d32("1.75"), RND_TOWARD_ZERO, flags!()));
}
