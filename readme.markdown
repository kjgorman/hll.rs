### [hll](http://kjgorman.github.io/hll.rs/basichll/struct.HLL.html)

![](https://travis-ci.org/kjgorman/hll.rs.svg?branch=master)

It's the hyperloglog cardinality estimation algorithm implemented in rust.

Specifically, it's the algorithm as presented on page 140 of the
journal the original (Flajolet et al.) paper was published in
([here](http://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf)). This
is unlike the [current HLL
crate](https://crates.io/crates/hyperloglog) on crates.io, which takes
into account the bias correction (as outlined
[here](http://stefanheule.com/papers/edbt2013-hyperloglog.pdf)).

Rust doc: [here](http://kjgorman.github.io/hll.rs/basichll/struct.HLL.html)

#### example

See the tests for some examples of usage, but broadly we can use it as
follows:

```rust
extern crate basichll as hll;
...
// Note the constructor parameter is the desired typical relative
// error, which dictates the size of the HLL instance. The error
// is 1.04 / sqrt(2 ^ bits), so for a 1% error you want about 12 bits
// -- which gives 2^12 size to it's underlying Vec (i.e. 4kb).
let mut log = hll::HLL::new(0.01625);

// you can insert anything that can be hashed
log.insert(&"foo");
log.insert(&"bar");
log.insert(&1);
log.insert(&2);

// you can count the approximate number of elements
println!("I think there are {} elements", log.count().round());
// would print "I think there are 4 elements"

// It implements Add so you can combine HLLs in a way the follows
// the monoid laws.

let first  = hll::HLL::new(0.26);
let second = hll::HLL::new(0.26);
let third  = &first + &second;
```