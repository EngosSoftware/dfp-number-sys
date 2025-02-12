use super::*;

#[test]
fn _0001() {
  let x = d64("25.4300");
  let mut exp = 0_i32;
  let z = bid64_frexp(x, &mut exp);
  eq("+254300E-6", z);
  assert_eq!(2, exp);
}
