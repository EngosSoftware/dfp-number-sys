use super::*;

#[test]
fn _0001() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma(d64("5"), d64("3"), d64("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0002() {
  let mut flags = EXE_CLEAR;
  eq("+1542E-2", bid64_fma(d64("3.3"), d64("5.4"), d64("-2.4"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0003() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_qqq(d128("5"), d128("3"), d128("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0004() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_ddq(d64("5"), d64("3"), d128("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0005() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_dqd(d64("5"), d128("3"), d64("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0006() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_dqq(d64("5"), d128("3"), d128("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0007() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_qdd(d128("5"), d64("3"), d64("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0008() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_qdq(d128("5"), d64("3"), d128("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}

#[test]
fn _0009() {
  let mut flags = EXE_CLEAR;
  eq("+17E+0", bid64_fma_qqd(d128("5"), d128("3"), d64("2"), RND_NEAREST_EVEN, &mut flags));
  eqf(EXE_CLEAR, flags);
}
