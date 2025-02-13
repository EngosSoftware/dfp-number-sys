use super::*;

#[test]
fn _0001() {
  let x = d32("25.4300");
  let mut exp = 0_i32;
  let z = bid32_frexp(x, &mut exp);
  eq("+254300E-6", z);
  assert_eq!(2, exp);
}
