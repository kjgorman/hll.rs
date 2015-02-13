#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

extern crate "basic-hll" as hll;

#[cfg(test)]
mod tests {
    use std::num::Float;
    use hll::*;

    #[test]
    fn adding_two_empty_hlls_results_in_an_empty_hll () {
        let first  = HLL::ctor(0.26);
        let second = HLL::ctor(0.26);
        let third  = first + second;

        assert_eq!(third.registers().iter().filter(|&r| *r != 0).count(), 0);
    }


    #[test]
    fn adding_together_two_hlls_preserves_the_sum_of_their_counts () {
        let mut first  = HLL::ctor(0.0040625);
        let mut second = HLL::ctor(0.0040625);

        first.insert(&"a"); first.insert(&"b"); first.insert(&"c");
        second.insert(&"d"); second.insert(&"e"); second.insert(&"f");

        assert_eq!(first.count().round(), 3.0);
        assert_eq!(second.count().round(), 3.0);

        let third = first + second;
        assert_eq!(third.count().round(), 6.0);
    }

    #[test]
    fn add_together_two_hlls_doesnt_double_count_duplicate_elements () {
        let mut first  = HLL::ctor(0.0040625);
        let mut second = HLL::ctor(0.0040625);

        first.insert(&"a"); first.insert(&"b"); first.insert(&"c");
        second.insert(&"a"); second.insert(&"d"); second.insert(&"e");

        assert_eq!(first.count().round(), 3.0);
        assert_eq!(second.count().round(), 3.0);

        let third = first + second;
        assert_eq!(third.count().round(), 5.0);
    }

    #[test]
    fn monoid_laws_should_hold_for_hll () {
        let zero = HLL::empty();
        let mut first  = HLL::ctor(0.0040625);
        let mut second = HLL::ctor(0.0040625);
        let mut third  = HLL::ctor(0.0040625);

        first.insert(&"foo");
        second.insert(&"bar");
        third.insert(&"quux");

        // left & right identity
        let leftZero  = first.clone() + zero.clone();
        let rightZero = zero.clone() + first.clone();

        assert_eq!(leftZero, first);
        assert_eq!(rightZero, first);
        // associativity
        let leftAssociate = (first.clone() + second.clone()) + third.clone();
        let rightAssociate = first.clone() + (second.clone() + third.clone());

        assert_eq!(leftAssociate, rightAssociate);
    }
}
