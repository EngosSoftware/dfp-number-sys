#![feature(test)]

extern crate test;

use dfp_number_sys::bid128_000::*;
use dfp_number_sys::*;
use test::Bencher;

#[bench]
fn bench_bid128_from_string_0001(b: &mut Bencher) {
  let mut flags = EXE_CLEAR;
  let round = RND_NEAREST_EVEN;
  b.iter(|| {
    let _ = bid128_from_string("128374.9458", round, &mut flags);
  });
}

#[bench]
fn bench_bid128_from_string_0002(b: &mut Bencher) {
  let mut flags = EXE_CLEAR;
  let round = RND_NEAREST_EVEN;
  b.iter(|| {
    let _ = bid128_from_string("-0.45985E-6", round, &mut flags);
  });
}

#[bench]
fn bench_bid128_from_string_0003(b: &mut Bencher) {
  let mut flags = EXE_CLEAR;
  let round = RND_NEAREST_EVEN;
  b.iter(|| {
    let _ = bid128_from_string("9999999999999999999999999999999999", round, &mut flags);
  });
}
