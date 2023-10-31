#![feature(test)]

extern crate test;

use dfp_number_sys::*;
use test::Bencher;

#[bench]
fn bench_bid128_add_0001(b: &mut Bencher) {
  let x = bid128_from_int32(2);
  let y = bid128_from_int32(5);
  let mut flags = FB_CLEAR;
  b.iter(|| {
    let _ = bid128_add(x, y, 0, &mut flags);
  });
}

#[bench]
fn bench_bid128_add_0002(b: &mut Bencher) {
  let x = bid128_scalbn(bid128_from_int64(235678910), -8);
  let y = bid128_scalbn(bid128_from_int64(235), -2);
  let mut flags = FB_CLEAR;
  b.iter(|| {
    let _ = bid128_add(x, y, 0, &mut flags);
  });
}
