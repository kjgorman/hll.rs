#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

use std::num::Float;
use std::io::BufferedReader;
use std::io::File;

mod hll;

#[test]
fn can_estimate_the_system_dictionary () {
    let path = Path::new("/usr/share/dict/words");
    let mut file = BufferedReader::new(File::open(&path));
    let mut store = hll::HLL::ctor(0.0040625);
    let mut count = 0f64;
    let actual = 60000.0f64;
    println!("store: {}", store);

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

