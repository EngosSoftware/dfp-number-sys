use super::*;

#[test]
fn _0001() {
  let x = bid64_scalbn(d64("2356789100"), -9);
  eq("+2356789100E-9", x);
}

#[test]
fn _0002() {
  let x = bid64_scalbn(d64("2356789100"), -9);
  let y = bid64_scalbn(x, 2);
  eq("+2356789100E-7", y);
}

#[test]
fn _0003() {
  let x = bid64_scalbn(d64("1"), 384);
  eq("+1000000000000000E+369", x);
}

#[test]
fn _0004() {
  let x = bid64_scalbn(d64("1"), 385);
  eq("+1000000000000000E+369", x);
}

#[test]
fn _0005() {
  let x = bid64_scalbn(d64("1"), -398);
  eq("+1E-398", x);
}

#[test]
fn _0006() {
  let x = bid64_scalbn(d64("1"), -399);
  eq("+1E-398", x);
}
