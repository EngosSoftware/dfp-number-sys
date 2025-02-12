use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+0E+0", bid128_expm1(d128("0"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1718281828459045235360287471352663E-33", bid128_expm1(d128("1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("-6321205588285576784044762298385391E-34", bid128_expm1(d128("-1"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_INEXACT, flags);
}
