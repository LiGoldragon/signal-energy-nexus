# Upgrades

## 0.3.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0, signal-flow 51.0

Repins signal 10.3.0 (`7afb`), datom-codec 0.33.0 (`eabe`), protos 0.33.0 (`3c7a`), the dev generator ethos-zero 19.2.0 (`5a5d`) and signal-flow 51.0 (`9f15`, a published branch revision, not a main).

- `Recipient` wraps signal-flow's `Address`, which in 51.0 names a model: `Address { aspect, topic, model }` replaces `Address { aspect, topic, layer }`, and `Layer` is gone. The Recipient wire form, binary and datom text, changes with it: `Mind energyNexus Secondary` becomes `Mind energyNexus Sol` or another `Model`. A consumer that builds a Recipient replaces `layer: Layer::…` with `model: Model::…`. The authored `signal.ethos` and the other wire types are unchanged.
- The unit tests add a binary and a datom-text round trip of a `Reminder` whose recipient names a model.
- Known and inherited: signal-flow 51.0 lists ethos-zero 17.0.0 (`07714b0`) as a build dependency, which carries protos 0.32.2 and datom-codec 0.32.2 into a consumer's lock beside the 0.33.0 pair this crate selects; the lock holds both until signal-flow repins that generator.
