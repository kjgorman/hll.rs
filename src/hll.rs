// Use actual greek variable names rather than latin (namely ρ vs. rho)
#![feature(non_ascii_idents)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]
#![crate_name = "basic-hll"]
#![crate_type = "lib"]

use std::cmp;
use std::fmt;
use std::num::Int;
use std::num::Float;
use std::hash;
use std::hash::Hash;

fn get_hash<Sized? T: Hash>(val: &T) -> u64 {
    hash::hash(val)
}

fn alpha (m: uint) -> f64 {
    match m {
        16 => 0.673,
        32 => 0.697,
        64 => 0.709,
        _  => 0.7213/(1f64 + 1.079/(m as f64))
    }
}

fn leftmost_one_bit (v: u64) -> uint {
    let mut counter: uint = 0;
    let mut shiftable = v;

    while shiftable > 0 {
        shiftable >>= 1;
        counter += 1;
    }

    64 - counter
}

pub struct HLL {
    alpha: f64,
    b: uint,
    m: uint,
    M: Vec<u8>
}

impl HLL {
    pub fn one_hundred_twenty_eight () -> HLL {
        HLL::ctor(0.09192)
    }
    
    pub fn ctor(error: f64) -> HLL {
        assert!(error > 0.0 && error < 1.0);
        // error = 1.04 / sqrt(m)
        let m = Float::floor((1.04/error) * (1.04/error)) as uint;
        let b = Float::log2(m as f64) as uint;
        
        HLL {
            alpha: alpha(m),
            b: b,
            m: m,
            M: Vec::from_elem(m, 0u8)
        }
    }

    pub fn insert<Sized? T: Hash>(&mut self, val: &T) {
        let hash = get_hash(val);
        // j is the first b many bits
        let j = (hash >> (64 - self.b)) as uint;
        // w is the remaining bits (i.e. b -> 64)
        let w = hash & (Int::pow(2, 64 - self.b) - 1);
        let ρ = leftmost_one_bit(w) as u8;

        self.M[j] = cmp::max(self.M[j], ρ);
    }

    pub fn count(&self) -> f64 {
        self.range_correction(self.raw_estimate())
    }

    fn raw_estimate(&self) -> f64 {
        let sum: f64 = self.M.iter().fold (0.0, |p, &r| p + 2.0f64.powi(-(r as i32)));

        self.alpha as f64 * Int::pow(self.m, 2) as f64 * (1.0 / sum)
    }

    fn empty_registers (&self) -> uint {
        self.M.iter().filter(|&r| *r == 0).count()
    }
    
    fn range_correction(&self, e: f64) -> f64 {
        let twoTo32: f64 = 2.0f64.powi(32) as f64;

        println!("estimate: {}", e);
        // small range correction
        if e <= (5.0*(self.m as f64))/2.0 {
            println!("small range correction");
            let v  = self.empty_registers() as f64;
            let fm = self.m as f64;
            
            return match v {
                0.0 => e,
                _   => fm * (fm / v).ln()
            }
        }

        // medium range (no) correction
        if e <= twoTo32/30.0 {
            println!("no correction");
            return e;
        }

        println!("large range correction");
        -twoTo32 * (1.0 - e/twoTo32).ln()
    }
}

impl fmt::Show for HLL {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "α: {}, b: {}, m: {}", self.alpha, self.b, self.m)
    }
}    
