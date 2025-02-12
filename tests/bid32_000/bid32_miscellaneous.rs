use super::*;

#[test]
fn bid12_debug_should_work() {
  assert_eq!("[32800001]", format!("{:?}", d32("1")));
}

#[test]
fn bid12_display_should_work() {
  assert_eq!("+1E+0", format!("{}", d32("1")));
}

#[test]
#[allow(clippy::clone_on_copy)]
fn bid128_clone_should_work() {
  let a = d32("1");
  let b = a.clone();
  assert_eq!(format!("{:?}", b), format!("{:?}", a));
}
