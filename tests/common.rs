mod tests_common {
  use dfp_number_sys::*;

  #[test]
  fn bid128_debug_should_work() {
    assert_eq!("BID128 { w: [1, 3476778912330022912] }", format!("{:?}", bid128_from_int32(1)));
  }

  #[test]
  #[allow(clippy::clone_on_copy)]
  fn bid128_clone_should_work() {
    let a = bid128_from_int32(1);
    let b = a.clone();
    assert_eq!(format!("{:?}", b), format!("{:?}", a));
  }
}
