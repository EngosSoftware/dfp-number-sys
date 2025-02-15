use super::*;

#[test]
fn _0001() {
  let value = bid128_nan("0");
  eq("+NaN", value);
  assert_eq!("[7C00000000000000,0000000000000000]", format!("{:?}", value));
}

#[test]
fn _0002() {
  let value = bid128_nan("1");
  eq("+NaN", value);
  assert_eq!("[7C00000000000000,0000000000000001]", format!("{:?}", value));
}

#[test]
fn _0003() {
  let value = bid128_nan("-39485E-4");
  eq("+NaN", value);
  assert_eq!("[7C00000000000000,0000000000009A3D]", format!("{:?}", value));
}
