// Use actual greek variable names rather than latin (namely ρ vs. rho)
#![feature(non_ascii_idents)]
// redesign
#![feature(hash)]
// pending
#![feature(core)]
// use variable names from paper, namely registers M
#![allow(non_snake_case)]

#![crate_name = "basic-hll"]
#![crate_type = "lib"]

/* -------------------- std libs ------------------- */
use std::cmp;
use std::fmt;
use std::hash::{ hash, Hash, SipHasher };
use std::iter;
use std::num::{ Int, Float };
/* ------------------------------------------------- */

/* -------------------- helpers -------------------- */

fn alpha (m: usize) -> f64 {
    match m {
        16 => 0.673,
        32 => 0.697,
        64 => 0.709,
        _  => 0.7213/(1f64 + 1.079/(m as f64))
    }
}

fn leftmost_one_bit (v: u64) -> usize {
    let mut counter: usize = 0;
    let mut shiftable = v;

    while shiftable > 0 {
        shiftable >>= 1;
        counter += 1;
    }

    64 - counter
}

/* ------------------------------------------------- */

/* --------------- hyper log log ------------------- */

pub struct HLL {
    alpha: f64,
    b: usize,
    m: usize,
    M: Vec<u8>,
    // [!] this is pretty much a hack until I can think of something
    //     better to do for the monoid instance. Basically we need the
    //     zero value to always satisfy the left and right identity laws
    //     but because you need to have the number of registers be
    //     equivalent to do addition, it implies you need a distinct
    //     monoid instance per HLL configuration (i.e. an HLL of 4
    //     registers has a distinct zero from a HLL of 16).
    //
    //     Because I can't really do dependent types in rust, we instead
    //     have a distinguished zero element, that is allowed to bypass
    //     the assertions on the equivalent sizes...
    //
    //     For reference, Ed Kmett's haskell implementation actually does
    //     use higher kinded types and some tricky reflection to make the
    //     monoid instance be parameterised by a config type that is itself
    //     typed to a Peano natural encoded type. Twitter's algebird instance
    //     declares a separate class that encapsulates the notion of
    //     a "HyperLogLogMonoid" instance, and instances of that class are
    //     instantiated with the register size (i.e. the 'instance' for monoid
    //     is just an 'instance' of the class, for two interpretations of
    //     'instance').
    isZero: bool
}

impl HLL {
    pub fn one_hundred_twenty_eight () -> HLL {
        HLL::ctor(0.09192)
    }

    pub fn ctor(error: f64) -> HLL {
        assert!(error > 0.0 && error < 1.0);
        // error = 1.04 / sqrt(m)
        let m = Float::floor((1.04/error) * (1.04/error)) as usize;
        let b = Float::log2(m as f64) as usize;

        HLL {
            alpha: alpha(m),
            b: b,
            m: m,
            M: iter::repeat(0u8).take(m).collect(),
            isZero: false
        }
    }

    pub fn insert<T: Hash>(&mut self, val: &T) {
        let hash = hash::<T, SipHasher>(val);

        // j is the first b many bits
        let j = (hash >> (64 - self.b)) as usize;
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

    fn empty_registers (&self) -> usize {
        self.M.iter().filter(|&r| *r == 0).count()
    }

    fn range_correction(&self, e: f64) -> f64 {
        let twoTo32: f64 = 2.0f64.powi(32) as f64;

        // small range correction
        if e <= (5.0*(self.m as f64))/2.0 {
            let v  = self.empty_registers() as f64;
            let fm = self.m as f64;

            return match v {
                0.0 => e,
                _   => fm * (fm / v).ln()
            }
        }

        // medium range (no) correction
        if e <= twoTo32/30.0 {
            return e;
        }

        -twoTo32 * (1.0 - e/twoTo32).ln()
    }

    pub fn registers(&self) -> Vec<u8> {
        self.M.clone()
    }

    pub fn clone (&self) -> HLL {
        HLL {
            alpha: self.alpha,
            b: self.b,
            m: self.m,
            M: self.M.clone(),
            isZero: self.isZero
        }
    }

    pub fn empty () -> HLL {
        HLL {
            alpha: 0.0,
            m: 0,
            b: 0,
            M: Vec::new(),
            isZero: true
        }
    }

}

/* ------------------------------------------------- */

/* --------------- trait instances ----------------- */

impl std::fmt::Display for HLL {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "α: {}, b: {}, m: {}", self.alpha, self.b, self.m)
    }
}

impl std::fmt::Debug for HLL {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "α: {}, b: {}, m: {}", self.alpha, self.b, self.m)
    }
}

impl std::cmp::Eq for HLL {}
impl std::cmp::PartialEq for HLL {
    fn eq(&self, other: &HLL) -> bool {
           self.alpha  == other.alpha
        && self.m      == other.m
        && self.b      == other.b
        && self.M      == other.M
        && self.isZero == other.isZero
    }
}

fn mergeRegisters (first: &Vec<u8>, second: &Vec<u8>) -> Vec<u8> {
    // TODO -- isn't this just zipWith max?
    let mut res: Vec<u8> = Vec::new();
    let mut zipper = first.iter().zip(second.iter());
    loop {
        match zipper.next() {
            None => return res,
            Some ((f, s)) => res.push(*cmp::max(f, s))
        }
    }
}

impl std::ops::Add<HLL> for HLL {
    type Output = HLL;

    fn add(self, other: HLL) -> HLL {
        // [!] gross...
        if self.isZero { return other.clone(); }
        if other.isZero { return self.clone(); }

        assert!(self.alpha == other.alpha);
        assert!(self.b == other.b);
        assert!(self.m == other.m);

        HLL {
            alpha: self.alpha,
            b: self.b,
            m: self.m,
            M: mergeRegisters(&self.M, &other.M),
            isZero: false
        }
    }
}

impl<'a> std::ops::Add<&'a HLL> for  HLL {
    type Output = HLL;

    fn add(self, other: &HLL) -> HLL {
        // [!] gross...
        if self.isZero { return other.clone(); }
        if other.isZero { return self.clone(); }

        assert!(self.alpha == other.alpha);
        assert!(self.b == other.b);
        assert!(self.m == other.m);

        HLL {
            alpha: self.alpha,
            b: self.b,
            m: self.m,
            M: mergeRegisters(&self.M, &other.M),
            isZero: false
        }
    }
}

/* ------------------------------------------------- */
