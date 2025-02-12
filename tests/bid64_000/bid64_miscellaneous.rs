use super::*;

#[test]
fn bid12_debug_should_work() {
  assert_eq!("[31C0000000000001]", format!("{:?}", d64("1")));
}

#[test]
fn bid12_display_should_work() {
  assert_eq!("+1E+0", format!("{}", d64("1")));
}

#[test]
#[allow(clippy::clone_on_copy)]
fn bid128_clone_should_work() {
  let a = d64("1");
  let b = a.clone();
  assert_eq!(format!("{:?}", b), format!("{:?}", a));
}
