use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("-Inf", bid128_log1p(d128("-1.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid128_log1p(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+6931471805599453094172321214581766E-34", bid128_log1p(d128("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq(
    "+1000000000000000000000000000000014E-33",
    bid128_log1p(d128("1.7182818284590452353602874713527"), RND_NEAREST_EVEN, &mut flags),
  );
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+2302585092994045684017991454684364E-33", bid128_log1p(d128("9.0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+Inf", bid128_log1p(d128("+Inf"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
