// use the literal ρ character rather than copping out with
// a variable named 'rho'
#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

mod hll;

fn main() {
    let mut x = hll::HLL::ctor(0.0040625);

    let words = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
                 "k", "l", "m", "n", "o", "p", "q", "r", "s", "t",
                 "u", "v", "q", "x", "y", "z"];

    for &word in words.iter() {
        x.insert(word);
    }
    
    println!("{}", x);
    println!("{}", x.count());
}
