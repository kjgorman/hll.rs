extern crate basichll as hll;

#[cfg(test)]
mod tests {
    use hll::*;

    #[test]
    fn serialize_deserialize_equal() {
        let first  = HLL::new(0.004);
        let second = HLL::from_vec(first.clone().into_vec().unwrap()).unwrap();

        assert!(first == second);
    }

    #[test]
    fn serialize_deserialize_count() {
        let mut first  = HLL::new(0.004);
        first.insert(&"1");
        first.insert(&"2");
        first.insert(&"3");
        first.insert(&"4");
        assert_eq!(first.count().round(), 4.0);

        let second = HLL::from_vec(first.clone().into_vec().unwrap()).unwrap();
        assert_eq!(second.count().round(), 4.0);
    }
}
