#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]
#![feature(test)]

extern crate "basic-hll" as hll;
extern crate test;

#[cfg(test)]
mod tests {
    use hll::*;
    use test::Bencher;


    #[bench]
    fn bench_string_addition(b: &mut Bencher) {
        b.iter(|| {
            let mut hll = HLL::ctor(0.0040625);
            let upper = 1000;
            let mut counter = 0;

            loop {
                if counter > upper {
                    break;
                }
                counter = counter + 1;
                hll.insert(&counter);
            }

            hll.count();
        });
    }

}
