//! This is a separate file to prevent tests from being included by the macros.

use crate::utils::{split_array, xor};

#[test]
fn xor_valid_result() {
    let mut source = [0, 1, 2, 3];
    xor(&mut source, &[34, 52]);

    assert_eq!(source, [34, 53, 32, 55]);
}

#[test]
fn xor_valid_result_with_one_byte() {
    let mut source = [0, 1, 2, 3];
    xor(&mut source, &[47]);

    assert_eq!(source, [47, 46, 45, 44]);
}

#[test]
fn xor_empty_key() {
    let mut source = [0, 1, 2, 3];
    xor(&mut source, &[]);

    assert_eq!(source, [0, 1, 2, 3]);
}

#[test]
fn xor_empty_source() {
    let mut source = [];
    xor(&mut source, &[45, 32, 56]);

    assert!(source.is_empty());
}

#[test]
fn split_array_valid_result() {
    let original = [1, 2, 3, 4, 5];
    let (left, right): ([u8; 2], [u8; 3]) = split_array(original);

    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5]);
}
