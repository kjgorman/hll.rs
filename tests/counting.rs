extern crate basichll as hll;

#[cfg(test)]
mod tests {
    use hll::*;

    #[test]
    fn a_hll_can_count_small_numbers () {
        let mut hll = HLL::new(0.0040625);

        assert!(hll.insert(&1));
        assert!(hll.insert(&2));
        assert!(hll.insert(&3));
        assert!(hll.insert(&1) == false);
        assert!(hll.insert(&2) == false);

        assert_eq!(hll.count().round(), 3.0);
    }

    #[test]
    fn a_hll_can_count_heterogenuous_items () {
        let mut hll = HLL::new(0.0040625);

        assert!(hll.insert(&1));
        assert!(hll.insert(&"foo"));
        assert!(hll.insert(&2));
        assert!(hll.insert(&"bar"));

        assert_eq!(hll.count().round(), 4.0);
    }
}
