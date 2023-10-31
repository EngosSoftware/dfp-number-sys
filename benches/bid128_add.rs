/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

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
