use dfp_number_sys::bid128_000::*;

#[test]
fn bid12_debug_should_work() {
  assert_eq!("[0x1, 0x3040000000000000]", format!("{:?}", bid128_from_int32(1)));
}

#[test]
fn bid12_display_should_work() {
  assert_eq!("+1E+0", format!("{}", bid128_from_int32(1)));
}

#[test]
#[allow(clippy::clone_on_copy)]
fn bid128_clone_should_work() {
  let a = bid128_from_int32(1);
  let b = a.clone();
  assert_eq!(format!("{:?}", b), format!("{:?}", a));
}
