#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

extern crate "basic-hll" as hll;

#[cfg(test)]
mod tests {
    use std::num::Float;
    use hll::*;

    #[test]
    fn a_hll_can_count_small_numbers () {
        let mut hll = HLL::ctor(0.0040625);

        hll.insert(&1);
        hll.insert(&2);
        hll.insert(&3);

        assert_eq!(hll.count().round(), 3.0);
    }

    #[test]
    fn a_hll_can_count_heterogenuous_items () {
        let mut hll = HLL::ctor(0.0040625);

        hll.insert(&1);
        hll.insert(&"foo");
        hll.insert(&2);
        hll.insert(&"bar");

        assert_eq!(hll.count().round(), 4.0);
    }
}
