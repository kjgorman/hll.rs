#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

extern crate "basic-hll" as hll;

#[test]
fn two_empty_hlls_are_equal() {
    let first  = hll::HLL::one_hundred_twenty_eight();
    let second = hll::HLL::one_hundred_twenty_eight();

    assert!(first == second);
}

#[test]
#[should_fail]
fn a_non_empty_hll_is_not_equal_to_an_empty_one() {
    let mut first = hll::HLL::one_hundred_twenty_eight();
    let second    = hll::HLL::one_hundred_twenty_eight();

    first.insert("foo");
    
    assert!(first == second);
}

#[test]
#[should_fail]
fn adding_two_different_elements_to_two_different_hlls_produces_differences() {
    let mut first  = hll::HLL::one_hundred_twenty_eight();
    let mut second = hll::HLL::one_hundred_twenty_eight();

    first.insert("foo");
    second.insert("bar");

    assert!(first == second);
}