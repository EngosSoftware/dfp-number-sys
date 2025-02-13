use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f64, bid64_to_binary64(d64("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  assert_eq!(0_f64, bid64_to_binary64(d64("-0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  assert!(f64::is_infinite(bid64_to_binary64(d64("+9999999999999999E+308"), RND_NEAREST_EVEN, &mut flags)));
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags1 = EXE_CLEAR;
  let mut flags2 = EXE_CLEAR;
  assert_eq!(
    f64::MAX,
    bid64_to_binary64(bid64_from_string("1.7976931348623157e308", RND_NEAREST_AWAY, &mut flags1), RND_TOWARD_ZERO, &mut flags2)
  );
  eqf(EXE_INEXACT, flags1);
  eqf(EXE_INEXACT | EXE_OVERFLOW, flags2);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  assert!(f64::is_infinite(bid64_to_binary64(d64("-9999999999999999E+308"), RND_NEAREST_EVEN, &mut flags)));
  eqf(EXE_OVERFLOW | EXE_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags1 = EXE_CLEAR;
  let mut flags2 = EXE_CLEAR;
  assert_eq!(
    f64::MIN,
    bid64_to_binary64(bid64_from_string("-1.7976931348623157e308", RND_NEAREST_AWAY, &mut flags1), RND_TOWARD_ZERO, &mut flags2)
  );
  eqf(EXE_INEXACT, flags1);
  eqf(EXE_INEXACT | EXE_OVERFLOW, flags2);
}
