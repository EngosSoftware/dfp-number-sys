use super::*;

#[test]
fn _bid32_round_integral_nearest_away() {
  eq("-2E+0", bid32_round_integral_nearest_away(d32("-2.0"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_away(d32("-1.9"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_away(d32("-1.8"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_away(d32("-1.7"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_away(d32("-1.6"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_away(d32("-1.5"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_away(d32("-1.4"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_away(d32("-1.3"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_away(d32("-1.2"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_away(d32("-1.1"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_away(d32("-1.0"), flags!()));
  eq("+1E+0", bid32_round_integral_nearest_away(d32("1.0"), flags!()));
  eq("+1E+0", bid32_round_integral_nearest_away(d32("1.1"), flags!()));
  eq("+1E+0", bid32_round_integral_nearest_away(d32("1.2"), flags!()));
  eq("+1E+0", bid32_round_integral_nearest_away(d32("1.3"), flags!()));
  eq("+1E+0", bid32_round_integral_nearest_away(d32("1.4"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("1.5"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("1.6"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("1.6"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("1.7"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("1.8"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("1.9"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_away(d32("2.0"), flags!()));
}

#[test]
fn _bid32_round_integral_exact_nearest_away() {
  eq("-2E+0", bid32_round_integral_exact(d32("-1.75"), RND_NEAREST_AWAY, flags!()));
  eq("-2E+0", bid32_round_integral_exact(d32("-1.5"), RND_NEAREST_AWAY, flags!()));
  eq("-1E+0", bid32_round_integral_exact(d32("-1.25"), RND_NEAREST_AWAY, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.25"), RND_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("1.5"), RND_NEAREST_AWAY, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("1.75"), RND_NEAREST_AWAY, flags!()));
}

#[test]
fn _bid32_round_integral_nearest_even() {
  eq("-2E+0", bid32_round_integral_nearest_even(d32("-2.0"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_even(d32("-1.9"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_even(d32("-1.8"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_even(d32("-1.7"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_even(d32("-1.6"), flags!()));
  eq("-2E+0", bid32_round_integral_nearest_even(d32("-1.5"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_even(d32("-1.4"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_even(d32("-1.3"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_even(d32("-1.2"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_even(d32("-1.1"), flags!()));
  eq("-1E+0", bid32_round_integral_nearest_even(d32("-1.0"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_even(d32("2.25"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_even(d32("2.5"), flags!()));
  eq("+3E+0", bid32_round_integral_nearest_even(d32("2.75"), flags!()));

  eq("+0E+0", bid32_round_integral_nearest_even(d32("0.5"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_even(d32("1.5"), flags!()));
  eq("+2E+0", bid32_round_integral_nearest_even(d32("2.5"), flags!()));
  eq("+4E+0", bid32_round_integral_nearest_even(d32("3.5"), flags!()));
  eq("+4E+0", bid32_round_integral_nearest_even(d32("4.5"), flags!()));
}

#[test]
fn _bid32_round_integral_exact_nearest_even() {
  eq("-3E+0", bid32_round_integral_exact(d32("-2.75"), RND_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid32_round_integral_exact(d32("-2.5"), RND_NEAREST_EVEN, flags!()));
  eq("-2E+0", bid32_round_integral_exact(d32("-2.25"), RND_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("2.25"), RND_NEAREST_EVEN, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("2.5"), RND_NEAREST_EVEN, flags!()));
  eq("+3E+0", bid32_round_integral_exact(d32("2.75"), RND_NEAREST_EVEN, flags!()));
}

#[test]
fn _bid32_round_integral_positive() {
  eq("-1E+0", bid32_round_integral_positive(d32("-1.75"), flags!()));
  eq("-1E+0", bid32_round_integral_positive(d32("-1.5"), flags!()));
  eq("-1E+0", bid32_round_integral_positive(d32("-1.25"), flags!()));
  eq("+2E+0", bid32_round_integral_positive(d32("1.25"), flags!()));
  eq("+2E+0", bid32_round_integral_positive(d32("1.5"), flags!()));
  eq("+2E+0", bid32_round_integral_positive(d32("1.75"), flags!()));
}

#[test]
fn _bid32_round_integral_exact_upward() {
  eq("-1E+0", bid32_round_integral_exact(d32("-1.75"), RND_UPWARD, flags!()));
  eq("-1E+0", bid32_round_integral_exact(d32("-1.5"), RND_UPWARD, flags!()));
  eq("-1E+0", bid32_round_integral_exact(d32("-1.25"), RND_UPWARD, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("1.25"), RND_UPWARD, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("1.5"), RND_UPWARD, flags!()));
  eq("+2E+0", bid32_round_integral_exact(d32("1.75"), RND_UPWARD, flags!()));
}

#[test]
fn _bid32_round_integral_negative() {
  eq("-2E+0", bid32_round_integral_negative(d32("-1.75"), flags!()));
  eq("-2E+0", bid32_round_integral_negative(d32("-1.5"), flags!()));
  eq("-2E+0", bid32_round_integral_negative(d32("-1.25"), flags!()));
  eq("+1E+0", bid32_round_integral_negative(d32("1.25"), flags!()));
  eq("+1E+0", bid32_round_integral_negative(d32("1.5"), flags!()));
  eq("+1E+0", bid32_round_integral_negative(d32("1.75"), flags!()));
}

#[test]
fn _bid32_round_integral_downward() {
  eq("-2E+0", bid32_round_integral_exact(d32("-1.75"), RND_DOWNWARD, flags!()));
  eq("-2E+0", bid32_round_integral_exact(d32("-1.5"), RND_DOWNWARD, flags!()));
  eq("-2E+0", bid32_round_integral_exact(d32("-1.25"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.25"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.5"), RND_DOWNWARD, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.75"), RND_DOWNWARD, flags!()));
}

#[test]
fn _bid32_round_integral_zero() {
  eq("-1E+0", bid32_round_integral_zero(d32("-1.75"), flags!()));
  eq("-1E+0", bid32_round_integral_zero(d32("-1.5"), flags!()));
  eq("-1E+0", bid32_round_integral_zero(d32("-1.25"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.0"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.1"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.2"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.3"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.4"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.5"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.6"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.7"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.8"), flags!()));
  eq("+1E+0", bid32_round_integral_zero(d32("1.9"), flags!()));
  eq("+2E+0", bid32_round_integral_zero(d32("2.0"), flags!()));
}

#[test]
fn _bid32_round_integral_toward_zero() {
  eq("-1E+0", bid32_round_integral_exact(d32("-1.75"), RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid32_round_integral_exact(d32("-1.5"), RND_TOWARD_ZERO, flags!()));
  eq("-1E+0", bid32_round_integral_exact(d32("-1.25"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.25"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.5"), RND_TOWARD_ZERO, flags!()));
  eq("+1E+0", bid32_round_integral_exact(d32("1.75"), RND_TOWARD_ZERO, flags!()));
}
