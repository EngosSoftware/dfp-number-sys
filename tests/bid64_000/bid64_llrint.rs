use super::*;

#[test]
fn _0001() {
  assert_eq!(-2_i64, bid64_llrint(d64("-1.75"), RND_NEAREST_AWAY, flags!()));
  assert_eq!(-2_i64, bid64_llrint(d64("-1.5"), RND_NEAREST_AWAY, flags!()));
  assert_eq!(-1_i64, bid64_llrint(d64("-1.25"), RND_NEAREST_AWAY, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.25"), RND_NEAREST_AWAY, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("1.5"), RND_NEAREST_AWAY, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("1.75"), RND_NEAREST_AWAY, flags!()));
}

#[test]
fn _0002() {
  assert_eq!(-3_i64, bid64_llrint(d64("-2.75"), RND_NEAREST_EVEN, flags!()));
  assert_eq!(-2_i64, bid64_llrint(d64("-2.5"), RND_NEAREST_EVEN, flags!()));
  assert_eq!(-2_i64, bid64_llrint(d64("-2.25"), RND_NEAREST_EVEN, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("2.25"), RND_NEAREST_EVEN, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("2.5"), RND_NEAREST_EVEN, flags!()));
  assert_eq!(3_i64, bid64_llrint(d64("2.75"), RND_NEAREST_EVEN, flags!()));
}

#[test]
fn _0003() {
  assert_eq!(-1_i64, bid64_llrint(d64("-1.75"), RND_UPWARD, flags!()));
  assert_eq!(-1_i64, bid64_llrint(d64("-1.5"), RND_UPWARD, flags!()));
  assert_eq!(-1_i64, bid64_llrint(d64("-1.25"), RND_UPWARD, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("1.25"), RND_UPWARD, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("1.5"), RND_UPWARD, flags!()));
  assert_eq!(2_i64, bid64_llrint(d64("1.75"), RND_UPWARD, flags!()));
}

#[test]
fn _0004() {
  assert_eq!(-2_i64, bid64_llrint(d64("-1.75"), RND_DOWNWARD, flags!()));
  assert_eq!(-2_i64, bid64_llrint(d64("-1.5"), RND_DOWNWARD, flags!()));
  assert_eq!(-2_i64, bid64_llrint(d64("-1.25"), RND_DOWNWARD, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.25"), RND_DOWNWARD, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.5"), RND_DOWNWARD, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.75"), RND_DOWNWARD, flags!()));
}

#[test]
fn _0005() {
  assert_eq!(-1_i64, bid64_llrint(d64("-1.75"), RND_TOWARD_ZERO, flags!()));
  assert_eq!(-1_i64, bid64_llrint(d64("-1.5"), RND_TOWARD_ZERO, flags!()));
  assert_eq!(-1_i64, bid64_llrint(d64("-1.25"), RND_TOWARD_ZERO, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.25"), RND_TOWARD_ZERO, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.5"), RND_TOWARD_ZERO, flags!()));
  assert_eq!(1_i64, bid64_llrint(d64("1.75"), RND_TOWARD_ZERO, flags!()));
}
