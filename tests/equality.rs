extern crate basichll as hll;

#[cfg(test)]
mod tests {
    use hll::*;

    #[test]
    fn two_empty_hlls_are_equal() {
        let first  = HLL::one_hundred_twenty_eight();
        let second = HLL::one_hundred_twenty_eight();

        assert!(first == second);
    }

    #[test]
    #[should_panic]
    fn a_non_empty_hll_is_not_equal_to_an_empty_one() {
        let mut first = HLL::one_hundred_twenty_eight();
        let second    = HLL::one_hundred_twenty_eight();

        first.insert(&"foo");

        assert!(first == second);
    }

    #[test]
    #[should_panic]
    fn adding_two_different_elements_to_two_different_hlls_produces_differences() {
        let mut first  = HLL::one_hundred_twenty_eight();
        let mut second = HLL::one_hundred_twenty_eight();

        first.insert(&"foo");
        second.insert(&"bar");

        assert!(first == second);
    }

}
