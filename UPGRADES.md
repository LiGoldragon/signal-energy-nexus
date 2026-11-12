# Upgrades

## 0.5.0 — signal 11.0.0 and signal-flow 61.0.0

Branch-only preparation for the Flow 0.54.7 rollout; deployed with it, in the same Home generation, with meta-signal-energy-nexus 0.5.0 and energy-nexus 0.3.2.

Repins signal 10.3.0 to 11.0.0 (`6d4eb74`), signal-flow 57.0.0 (`3d0a4cd`) to 61.0.0 (`4871bb8`, the contract Flow 0.54.7 is built with), datom-codec to `09a0526` and protos to `c93b598` (the pair signal-flow 61 builds with). `signal` declares `links = "signal"`, so a consumer carries one signal: this contract moves with signal-flow 61, which depends on signal 11.0.0.

The Rust types change and the wire does not. `Recipient` wraps signal-flow's `Address`, whose `topic` is now the `Topic` newtype (`Topic("energyNexus".into())`); the binary and datom text of a Recipient and of every other type are unchanged, and the authored `signal.ethos` is unchanged, so the contract digest of this crate's own types does not move. No Ethos output is generated from `signal.ethos` in this repository (it is included as text and checked by a test); nothing to regenerate.

`tests/wire_fixtures.rs` holds the wire against `tests/fixtures/`, made by 97741d2 (0.4.0): the rkyv bytes and the datom text of a value of every wire type and every variant, the contract digest, and the greeting and receipt that carry it. They are identical under 0.5.0.

## 0.4.0 — signal-flow 57.0.0 with Observe.TurnEnds; Read answers the latest reading

Repins signal-flow 57.0.0 at `3d0a4cd` (Observe.TurnEnds, the turn ends the Energy Nexus reads a provider on, and Observe.Metaflows; branch idle-0533-ff3882, not yet a main). The wire types are unchanged. `Read.Provider` now answers the provider's latest reading, read from the provider first when that reading is older than the last turn end on it (or there is none); before, every `Read` read the provider. The authored `signal.ethos` says so on the `Read` line, so the contract digest changes.

## 0.3.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0, signal-flow 57.0.0

Repins signal 10.3.0 (`7afb`), datom-codec 0.33.0 (`eabe`), protos 0.33.0 (`3c7a`), the dev generator ethos-zero 19.2.0 (`5a5d`) and signal-flow 57.0.0 (`065730f`, the contract Flow 0.53.3 is built with; branch idle-0533-ff3882, not yet a main).

- `Recipient` wraps signal-flow's `Address`, which since 51.0 names a model (in 57.0.0 it is flow-ethos's `Address.{ Aspect Topic Model }`, re-exported): `Address { aspect, topic, model }` replaces `Address { aspect, topic, layer }`, and `Layer` is gone. The Recipient wire form, binary and datom text, changes with it: `Mind energyNexus Secondary` becomes `Mind energyNexus Sol` or another `Model`. A consumer that builds a Recipient replaces `layer: Layer::…` with `model: Model::…`. The authored `signal.ethos` and the other wire types are unchanged.
- The unit tests add a binary and a datom-text round trip of a `Reminder` whose recipient names a model.
- Known and inherited: signal-flow 57.0.0 and its flow-ethos list ethos-zero 17.0.0 (`07714b0`) as a build dependency, which carries protos 0.32.2 and datom-codec 0.32.2 into a consumer's lock beside the 0.33.0 pair this crate selects; the lock holds both until signal-flow repins that generator.
