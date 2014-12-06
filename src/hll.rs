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

    counter
}

pub struct HLL {
    alpha: f64,
    b: uint,
    m: uint,
    M: Vec<u8>
}

impl HLL {
    pub fn ctor(error: f64) -> HLL {
        assert!(error > 0.0 && error < 1.0)
            // error = 1.04 / sqrt(m)
            let m = Float::floor((1.04/error) * (1.04/error)) as uint;
        let b = Float::log2(m as f64) as uint;
        
        HLL {
            alpha: alpha(m),
            b: b,
            m: m,
            M: Vec::from_elem(b, 0u8)
        }
    }

    pub fn insert<Sized? T: Hash>(&mut self, val: &T) {
        let hash = get_hash(val);
        let j = (hash & (self.b - 1) as u64) as uint;
        let w = hash >> self.b;
        let ρ = leftmost_one_bit(w) as u8;

        self.M[j] = cmp::max(self.M[j], ρ);
    }

    pub fn count(&self) -> f64 {
        self.range_correction(self.raw_estimate())
    }

    fn raw_estimate(&self) -> f64 {
        let sum: i64 = self.M.iter().fold (0, |p, &r| p + Int::pow(2, -1 * (r as uint)));
        
        self.alpha as f64 * Int::pow(self.m, 2) as f64 * (1.0 / sum as f64)
    }

    fn empty_registers (&self) -> uint {
        self.M.iter().filter(|&r| *r == 0).count()
    }
    
    fn range_correction(&self, e: f64) -> f64 {
        let twoTo32: f64 = (1u << 32u) as f64;

        // small range correction
        if e <= (5.0*(self.m as f64))/2.0 {
            let v  = self.empty_registers() as f64;
            let fm = self.m as f64;
            
            return match v {
                0.0 => e,
                _   => fm * Float::log10(fm / v)
            }
        }

        // medium range correction
        if (30.0 * e) <= twoTo32 {
            return e;
        }

        // TODO -- what's the base of this log?
        -1.0 * twoTo32 * Float::log10(1.0 - e/twoTo32)
    }
}

impl fmt::Show for HLL {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "α: {}, b: {}, m: {}, M: {}", self.alpha, self.b, self.m, self.M)
    }
}    
