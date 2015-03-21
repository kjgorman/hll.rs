// these are all unstable...
#![feature(core)]

extern crate "basic-hll" as hll;

#[cfg(test)]
mod tests {

    use std::num::{ Int, Float };
    use std::io::{ BufRead, BufReader };
    use std::fs::File;
    use std::path::Path;

    use hll::*;

    #[test]
    fn can_estimate_a_small_range_subset_of_the_system_dictionary () {
        let path = Path::new("/usr/share/dict/words");
        let file = BufReader::new(File::open(&path).unwrap());
        let mut store = HLL::new(0.0040625);
        let mut count = 0f64;
        let actual = 60000.0f64;

        for line in file.lines() {
            count += 1.0;
            if count > actual {
                break;
            }
            store.insert(&line.unwrap());
        }

        let count = store.count();
        let error = 1.04 / 65536.0f64.sqrt();

        if  (1.0 - (count / actual)) > error {
            panic!("expected {} to be within {} of {} (but was {})", count, error, actual, 1.0 - (count / actual));
        }
    }

    #[test]
    fn can_estimate_a_large_sequence_of_floating_points () {
        let limit = 2i64.pow(20);
        let mut store = HLL::new(0.0040625);
        let mut counter = 0i64;

        loop {
            store.insert(&counter);
            counter += 1;
            if counter > limit {
                break;
            }
        }

        let count = store.count();
        let error = 1.04 / 65536.0f64.sqrt();

        if (1.0 - (count / limit as f64)).abs() > error {
            panic!("expected {} to be within {} of {} (but was {})"
                   , count, error, limit, 1.0 - (count / limit as f64).abs());
        }
    }

}
