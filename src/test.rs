#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

use std::io::BufferedReader;
use std::io::File;

mod hll;

#[test]
fn can_estimate_the_system_dictionary () {
    let path = Path::new("/usr/share/dict/words");
    let mut file = BufferedReader::new(File::open(&path));
    let mut store = hll::HLL::ctor(0.13);
    
    for line in file.lines() {
        store.insert(&line.unwrap());
    }

    if store.count() != 235886.0 {
        panic!("expected to get 235886 but got <{}>", store.count());
    }
}

