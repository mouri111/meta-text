#![feature(test)]

extern crate meta_text;
extern crate test;

use meta_text::entry::*;
use test::Bencher;

#[bench]
fn bench_1(b: &mut Bencher) {
    b.iter(|| {
        entry();
    });
}
