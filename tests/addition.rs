#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

extern crate "basic-hll" as hll;
extern crate algebra;

use std::num::Float;
use algebra::structure::IdentityAdditive;

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

#[test]
fn adding_together_two_hlls_preserves_the_sum_of_their_counts () {
    let mut first  = hll::HLL::ctor(0.0040625);
    let mut second = hll::HLL::ctor(0.0040625);

    first.insert("a"); first.insert("b"); first.insert("c");
    second.insert("d"); second.insert("e"); second.insert("f");

    assert_eq!(first.count().round(), 3.0);
    assert_eq!(second.count().round(), 3.0);

    let third = first + second;
    assert_eq!(third.count().round(), 6.0);
}

#[test]
fn add_together_two_hlls_doesnt_double_count_duplicate_elements () {
    let mut first  = hll::HLL::ctor(0.0040625);
    let mut second = hll::HLL::ctor(0.0040625);

    first.insert("a"); first.insert("b"); first.insert("c");
    second.insert("a"); second.insert("d"); second.insert("e");

    assert_eq!(first.count().round(), 3.0);
    assert_eq!(second.count().round(), 3.0);

    let third = first + second;
    assert_eq!(third.count().round(), 5.0);
}

#[test]
fn monoid_laws_should_hold_for_hll () {
    let zero: hll::HLL = IdentityAdditive::zero();
    let mut first  = hll::HLL::ctor(0.0040625);
    let mut second = hll::HLL::ctor(0.0040625);
    let mut third  = hll::HLL::ctor(0.0040625);
    
    first.insert("foo");
    second.insert("bar");
    third.insert("quux");

    // left & right identity
    assert!(zero + first == first);
    assert!(first + zero == first);
    //associativity
    assert!((first + second) + third == first + (second + third));
}
