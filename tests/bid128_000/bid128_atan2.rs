use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  let result = bid128_atan2(d128("0.0"), d128("0.0"), RND_TOWARD_ZERO, &mut flags);
  eq("+0E-6176", result);
  bid128_is_zero(result);
  assert_eq!(flags, EXE_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq(
    "+7853981633974483096156608458198756E-34",
    bid128_atan2(d128("1.0"), d128("1.0"), RND_TOWARD_ZERO, &mut flags),
  );
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq(
    "-2356194490192344928846982537459627E-33",
    bid128_atan2(d128("-1.0"), d128("-1.0"), RND_TOWARD_ZERO, &mut flags),
  );
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq(
    "-7853981633974483096156608458198756E-34",
    bid128_atan2(d128("-1.0"), d128("1.0"), RND_TOWARD_ZERO, &mut flags),
  );
  assert_eq!(flags, EXE_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq(
    "+2356194490192344928846982537459627E-33",
    bid128_atan2(d128("1.0"), d128("-1.0"), RND_TOWARD_ZERO, &mut flags),
  );
  assert_eq!(flags, EXE_INEXACT);
}
