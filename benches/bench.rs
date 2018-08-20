#![feature(test)]

extern crate etxt;
extern crate test;

use etxt::*;
use test::Bencher;

#[bench]
fn bench_1(b: &mut Bencher) {
    b.iter(|| {
        entry();
    });
}
