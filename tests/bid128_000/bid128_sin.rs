use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  eq("+0E-1", bid128_sin(d128("0.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+8414709848078965066525023216302991E-34", bid128_sin(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-8414709848078965066525023216302991E-34", bid128_sin(d128("-1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("-4102067615373566167089928953969909E-43", bid128_sin(d128("3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("+9999999999999999999789663015986293E-34", bid128_sin(d128("1.570796327"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}
