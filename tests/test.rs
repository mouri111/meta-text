extern crate etxt;

use std::fs;
use std::io::{Read};
use etxt::*;

#[test]
fn test_1() {
    entry();
}

fn read_file(filename: &str) -> String {
    let mut file = fs::File::open(filename).unwrap();
    let mut ss = String::new();
    file.read_to_string(&mut ss).unwrap();
    ss
}

#[test]
fn test_read_file() {
    let ss = read_file("tests/testcases/in001.etxt");
    eprintln!("{}", ss);
    assert_eq!(ss, "\"abcd\\n\";\n");
}

#[test]
fn test_2() {
    let input = read_file("tests/testcases/in001.etxt");
    let exp_output = read_file("tests/testcases/out001.txt");
    let output = etxt::render(input);
    assert_eq!(exp_output, output);
}

#[test]
fn test_3() {
    let input = read_file("tests/testcases/in002.etxt");
    let exp_output = read_file("tests/testcases/out002.txt");
    let output = etxt::render(input);
    assert_eq!(exp_output, output);
}
