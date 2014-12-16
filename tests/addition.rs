#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

extern crate "basic-hll" as hll;

#[test]
fn adding_two_empty_hlls_results_in_an_empty_hll () {
    let first  = hll::HLL::ctor(0.26);
    let second = hll::HLL::ctor(0.26);
    let third  = first + second;

    assert_eq!(third.registers().iter().filter(|&r| *r != 0).count(), 0);
}

#[test]
fn adding_together_two_hlls_takes_the_max_in_each_register () {
    let mut first  = hll::HLL::ctor(0.26);
    let mut second = hll::HLL::ctor(0.26);

    first.insert("foo");  // sets register 14 to 7
    second.insert("bar"); // sets register 6 to 5

    let third = first + second;
    assert_eq!(third.registers()[14], 7);
    assert_eq!(third.registers()[6], 5);
}
