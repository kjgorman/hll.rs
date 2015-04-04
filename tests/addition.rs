extern crate basichll as hll;

#[cfg(test)]
mod tests {
    use hll::*;

    #[test]
    fn adding_two_empty_hlls_results_in_an_empty_hll () {
        let first  = HLL::new(0.26);
        let second = HLL::new(0.26);
        let third  = first + second;

        assert_eq!(third.registers().iter().filter(|&r| *r != 0).count(), 0);
    }


    #[test]
    fn adding_together_two_hlls_preserves_the_sum_of_their_counts () {
        let mut first  = HLL::new(0.0040625);
        let mut second = HLL::new(0.0040625);

        first.insert(&"a"); first.insert(&"b"); first.insert(&"c");
        second.insert(&"d"); second.insert(&"e"); second.insert(&"f");

        assert_eq!(first.count().round(), 3.0);
        assert_eq!(second.count().round(), 3.0);

        let third = first + second;
        assert_eq!(third.count().round(), 6.0);
    }

    #[test]
    fn add_together_two_hlls_doesnt_double_count_duplicate_elements () {
        let mut first  = HLL::new(0.0040625);
        let mut second = HLL::new(0.0040625);

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
        let mut first  = HLL::new(0.0040625);
        let mut second = HLL::new(0.0040625);
        let mut third  = HLL::new(0.0040625);

        first.insert(&"foo");
        second.insert(&"bar");
        third.insert(&"quux");

        // left & right identity
        let left_zero  = first.clone() + zero.clone();
        let right_zero = zero.clone() + first.clone();

        assert_eq!(left_zero, first);
        assert_eq!(right_zero, first);
        // associativity
        let left_associate = (first.clone() + second.clone()) + third.clone();
        let right_associate = first.clone() + (second.clone() + third.clone());

        assert_eq!(left_associate, right_associate);
    }
}
