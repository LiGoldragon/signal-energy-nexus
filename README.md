# signal-energy-nexus

The typed, rkyv-validated ordinary wire contract for Energy Nexus v1.

Every type is generated from `ethos/library.ethos` and `ethos/signal.ethos`
by ethos-zero (a build dependency); `build.rs` refuses a committed
`src/generated/*.rs` that differs from its generation.
