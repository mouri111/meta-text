extern crate etxt;

use std::fs;
use std::io::{Read};
use etxt::sandbox::*;

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
    let ss = read_file("tests/testcases/001.in.etxt");
    assert_eq!(ss, "\"abcd\\n\";\n");
}

#[test]
fn test_4() {
    for i in 1..10+1 {
        let input_filename = format!("tests/testcases/{: >03}.in.etxt", i);
        let input = read_file(input_filename.as_str());
        let exp_output_filename = format!("tests/testcases/{: >03}.out.txt", i);
        let exp_output = read_file(exp_output_filename.as_str());
        let output = render(input);
        assert_eq!(exp_output, output);
    }
}
