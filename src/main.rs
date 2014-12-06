// use the literal ρ character rather than copping out with
// a variable named 'rho'
#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

mod hll;

fn main() {
    let mut x = hll::HLL::ctor(0.13);

    x.insert("foo");
    println!("{}", x.count());
}
