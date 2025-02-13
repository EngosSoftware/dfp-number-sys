use super::*;

#[test]
fn _0001() {
  let x = bid128_scalbn(d128("2356789100"), -9);
  eq("+2356789100E-9", x);
}

#[test]
fn _0002() {
  let x = bid128_scalbn(d128("2356789100"), -9);
  let y = bid128_scalbn(x, 2);
  eq("+2356789100E-7", y);
}

#[test]
fn _0003() {
  let x = bid128_scalbn(d128("1"), 6144);
  eq("+1000000000000000000000000000000000E+6111", x);
}

#[test]
fn _0004() {
  let x = bid128_scalbn(d128("1"), 6145);
  eq("+1000000000000000000000000000000000E+6111", x);
}

#[test]
fn _0005() {
  let x = bid128_scalbn(d128("1"), -6176);
  eq("+1E-6176", x);
}

#[test]
fn _0006() {
  let x = bid128_scalbn(d128("1"), -6177);
  eq("+1E-6176", x);
}
