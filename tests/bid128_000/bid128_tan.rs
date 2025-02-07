use super::*;
use dfp_number_sys::bid128_000::*;

#[test]
fn _0001() {
  let mut flags = FB_CLEAR;
  let result = bid128_tan(d128("0.0"), RM_NEAREST_EVEN, &mut flags);
  eq("+0E-41", result);
  assert!(bid128_is_zero(result));
  assert_eq!(flags, FB_CLEAR);
}

#[test]
fn _0002() {
  let mut flags = FB_CLEAR;
  eq("+1557407724654902230506974807458360E-33", bid128_tan(d128("1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0003() {
  let mut flags = FB_CLEAR;
  eq("-1557407724654902230506974807458360E-33", bid128_tan(d128("-1.0"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0004() {
  let mut flags = FB_CLEAR;
  eq("+4102067615373566167435055566145099E-43", bid128_tan(d128("3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0005() {
  let mut flags = FB_CLEAR;
  eq("-4102067615373566167435055566145099E-43", bid128_tan(d128("-3.141592654"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0006() {
  let mut flags = FB_CLEAR;
  eq("+9262049631670410244970208729006011E-32", bid128_tan(d128("1.56"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}

#[test]
fn _0007() {
  let mut flags = FB_CLEAR;
  eq("-1086492036048439344550543836465099E-31", bid128_tan(d128("1.58"), RM_NEAREST_EVEN, &mut flags));
  assert_eq!(flags, FB_INEXACT);
}
