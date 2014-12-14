### hll

It's the hyperloglog cardinality estimation algorithm implemented in rust.

Specifically, it's the algorithm as presented on page 140 of the
journal the original (Flajolet et al.) paper was published in
([here](http://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf)). This
is unlike the [current HLL
crate](https://crates.io/crates/hyperloglog) on crates.io, which takes
into account the bias correction (as outlined
[here](http://stefanheule.com/papers/edbt2013-hyperloglog.pdf)).
