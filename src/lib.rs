// use variable names from paper, namely registers M
#![allow(non_snake_case)]

#![crate_name = "basichll"]
#![crate_type = "lib"]

/* -------------------- std libs ------------------- */
use std::cmp;
use std::fmt;
use std::hash::{ Hash, Hasher, SipHasher };
use std::iter::{ repeat, Iterator };
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

/// The `HLL` struct stores the underlying registers
/// of the HyperLogLog, with `m` many registers.
///
/// The value for `m` is derived from the desired error
/// of the estimation. The error is considered to be
/// 1.04 / sqrt(2 ^ bits). So, given a desired 1% error
/// rate, we result in around 12 bits per register,
/// producing an underlying M of 2^12 bits (i.e. 4kb).
pub struct HLL {
    alpha: f64,
    b: u32,
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
    /// Convenience function to produce a HLL with
    /// one hundred and twenty eight registers.
    pub fn one_hundred_twenty_eight () -> HLL {
        HLL::new(0.09192)
    }

    /// Create a new HLL with the desired standard error.
    /// Some examples might be:
    ///
    ///```ignore
    ///     bits    size    error
    ///     12      4096    0.0163
    ///     13      8192    0.0115
    ///     14      16384   0.0081
    ///```
    ///
    /// The error must be between 0.0 and 1.0. Beware using a stupidly
    /// small error size will grow beyond e.g. isize at less than 0.03
    /// (if you need something that isn't an estimate probably don't use
    /// a hyperloglog).
    pub fn new(error: f64) -> HLL {
        assert!(error > 0.0 && error < 1.0);
        // error = 1.04 / sqrt(m)
        let m = ((1.04/error) * (1.04/error)).floor() as usize;
        let b = (m as f64).log2() as u32;

        HLL {
            alpha: alpha(m),
            b: b,
            m: m,
            M: repeat(0u8).take(m).collect(),
            isZero: false
        }
    }

    /// Add an element into the hyperloglog estimate.
    /// We require the type of value to be able to be hashed.
    /// Returns whether the insertion altered the hyperloglog
    pub fn insert<T: Hash>(&mut self, val: &T) -> bool {
        let mut hasher = SipHasher::new();
        val.hash(&mut hasher);
        let hash = hasher.finish();

        // j is the first b many bits
        let j = (hash >> (64 - self.b)) as usize;
        // w is the remaining bits (i.e. b -> 64)
        let w = hash & (2u64.pow(64 - self.b) - 1) as u64;
        let rho = leftmost_one_bit(w) as u8;

        let prev = self.M[j];
        self.M[j] = cmp::max(self.M[j], rho);

        prev != self.M[j]
    }

    /// Return the estimated cardinality of the observed set
    /// of elements.
    pub fn count(&self) -> f64 {
        self.range_correction(self.raw_estimate())
    }

    fn raw_estimate(&self) -> f64 {
        let sum: f64 = self.M.iter().fold (0.0, |p, &r| p + 2.0f64.powi(-(r as i32)));

        self.alpha as f64 * ((self.m  * self.m) as f64) * (1.0 / sum)
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

    /// Access a copy of the underlying registers (this is mainly
    /// here just for debugging/testing... its unlikely you'll need
    /// this access typically).
    pub fn registers(&self) -> Vec<u8> {
        self.M.clone()
    }

    /// Copies this instance into a new HLL.
    /// Doesn't do anything tricky (i.e. this will entirely
    /// re-allocate the underlying registers).
    pub fn clone (&self) -> HLL {
        HLL {
            alpha: self.alpha,
            b: self.b,
            m: self.m,
            M: self.M.clone(),
            isZero: self.isZero
        }
    }

    /// A completely zeroed HLL. Not particularly useful
    /// except as an identity element in `Add`.
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

/// A HLL is considered equal to another HLL when its
/// configuration and registers are exactly identical.
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

fn zipWith<R, U: Iterator, C: Fn(U::Item, U::Item) -> R> (combo: C, left: U, right: U)
   -> Vec<R> {
    left.zip(right).map(| (l, r) | combo(l, r)).collect()
}

fn mergeRegisters (first: &Vec<u8>, second: &Vec<u8>) -> Vec<u8> {
    zipWith(|&l, &r| std::cmp::max(l, r), first.iter(), second.iter())
}

/// Adding together two HLLs produces a new HLL where
/// the larger value of each register has been selected.
impl<'a> std::ops::Add<&'a HLL> for &'a HLL {
    type Output = HLL;

    fn add(self, other: &'a HLL) -> HLL {
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
