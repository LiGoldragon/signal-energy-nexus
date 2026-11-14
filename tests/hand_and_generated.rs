//! The hand-written contract of 0.5.0 (7ca10eb) against the contract
//! ethos-zero now generates. `hand` is the 0.5.0 `src/lib.rs`, frozen as it
//! stood. A value of every type and of every variant is archived through
//! the hand type; its bytes are read as the generated type and archived
//! again, and the two archives must be one; the generated archive must read
//! back as the hand value; and under `datom` the two texts must be one.
//!
//! The one difference is the order of the outputs. 0.6.0 moves `Redeemed`
//! before `Changed`, so that `Redeemed` is the third output and answers the
//! third input, `Redeem`, as the generated `Answering` pairs them. So
//! `Output::Redeemed` archives with tag 2 and `Output::Changed` with tag 3,
//! where 0.5.0 had 3 and 2. Every output is archived through the hand type
//! and the generated variant of the same name: every byte is one but that tag,
//! and the two datom texts, which carry names, are one.
//!
//! `Target` and `Pace` change shape in 0.6.0 by design (`SpendBy` carries
//! a percent; pace is counted in useful hours), so neither is proved here:
//! `tests/wire_fixtures.rs` holds their 0.6.0 bytes and text. A `Read_Data`
//! and `Output::Read` are proved with no pace, which they archive as before.
//!
//! The contract digest is compared last: it is the hash of the authored
//! Ethos text, and the text had to change, since ethos-zero 28 refuses the
//! 0.5.0 text (`Duplicate.Input`: a Signal implies its Input).

use signal::Contracted;
use std::fmt::Debug;

#[allow(non_camel_case_types, dead_code)]
mod hand {
    macro_rules! wire {
        ($item:item) => {
            #[derive(
                rkyv::Archive,
                rkyv::Serialize,
                rkyv::Deserialize,
                Clone,
                Debug,
                PartialEq,
                Eq,
                Hash,
            )]
            #[cfg_attr(
                feature = "datom",
                derive(datom_codec::Datomizable, datom_codec::Composing)
            )]
            $item
        };
    }
    wire! { pub enum Provider { Claude, Codex } }
    wire! { pub struct Percent(pub i64); }
    wire! { pub struct Instant(pub i64); }
    wire! { pub struct Minutes(pub i64); }
    wire! { pub enum Name { Session, Week, FableWeek } }
    wire! { pub struct Window { pub name: Name, pub used: Percent, pub resets: Instant, pub duration: Minutes } }
    wire! { pub struct ResetId(pub String); }
    wire! { pub enum ResetStatus { Available, Redeeming, Redeemed } }
    wire! { pub struct Reset { pub reset_id: ResetId, pub reset_status: ResetStatus, pub expires: Instant } }
    wire! { pub struct Reading { pub provider: Provider, pub allowed: bool, pub window_vector: Vec<Window>, pub reset_vector: Vec<Reset>, pub at: Instant } }
    wire! { pub enum Level { Plenty, Fair, Low, Out } }
    wire! { pub struct Threshold { pub level: Level, pub left: Percent } }
    pub use signal_flow::Address;
    wire! { pub struct Recipient(pub Address); }
    wire! { pub struct Reminder { pub provider: Provider, pub level: Level, pub recipient: Recipient } }
    wire! { pub enum Target { Even, SpendBy(Instant) } }
    wire! { pub enum Standing { Over, Under, On } }
    wire! { pub struct Pace { pub provider: Provider, pub name: Name, pub target: Target, pub expected: Percent, pub used: Percent, pub standing: Standing, pub per_day: Percent } }
    wire! { pub struct Caller { pub process_id: i64, pub user_id: i64 } }
    wire! { pub enum Query { Read(Provider), Subscribe, Redeem(Provider) } }
    wire! { pub struct Call { pub caller: Caller, pub input: Query } }
    wire! { pub struct Read_Data { pub reading: Reading, pub pace_vector: Vec<Pace> } }
    wire! { pub struct Changed_Data { pub provider: Provider, pub level: Level, pub reading: Reading } }
    wire! { pub enum Refused { ProviderDown(Provider), NoReset, RuleRefused(String) } }
    wire! { pub enum Response { Read(Read_Data), Subscribed, Changed(Changed_Data), Redeemed(Reset), Refused(Refused) } }
    pub const ETHOS: &str = include_str!("fixtures/signal-0.5.0.ethos");
    impl signal::Contracted for Call {
        const CONTRACT_SOURCE: &'static str = ETHOS;
    }
}

use signal_energy_nexus as generated;

type Strict = rkyv::rancor::Error;

/// Every difference found, by the name of the case.
struct Proof {
    differences: Vec<String>,
    cases: usize,
}

trait Proving<Hand, Generated> {
    fn prove(&mut self, name: &str, hand: Hand);
}

impl<Hand, Generated> Proving<Hand, Generated> for Proof
where
    Hand: rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                Strict,
            >,
        > + PartialEq
        + Debug
        + Datom,
    Hand::Archived: for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, Strict>>
        + rkyv::Deserialize<Hand, rkyv::api::high::HighDeserializer<Strict>>,
    Generated: rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                Strict,
            >,
        > + Debug
        + Datom,
    Generated::Archived: for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, Strict>>
        + rkyv::Deserialize<Generated, rkyv::api::high::HighDeserializer<Strict>>,
{
    fn prove(&mut self, name: &str, hand: Hand) {
        self.cases += 1;
        let hand_bytes = rkyv::to_bytes::<Strict>(&hand).unwrap();
        let generated = match rkyv::from_bytes::<Generated, Strict>(&hand_bytes) {
            Ok(value) => value,
            Err(error) => {
                self.differences
                    .push(format!("{name}: the hand bytes do not read as the generated type: {error}"));
                return;
            }
        };
        let generated_bytes = rkyv::to_bytes::<Strict>(&generated).unwrap();
        if hand_bytes.as_slice() != generated_bytes.as_slice() {
            self.differences.push(format!(
                "{name}: hand {:02x?} generated {:02x?}",
                hand_bytes.as_slice(),
                generated_bytes.as_slice()
            ));
        }
        match rkyv::from_bytes::<Hand, Strict>(&generated_bytes) {
            Ok(back) if back == hand => {}
            Ok(back) => self
                .differences
                .push(format!("{name}: the generated bytes read back as {back:?}, not {hand:?}")),
            Err(error) => self
                .differences
                .push(format!("{name}: the generated bytes do not read as the hand type: {error}")),
        }
        let (hand_text, generated_text) = (hand.text(), generated.text());
        if hand_text != generated_text {
            self.differences.push(format!(
                "{name}: hand text {hand_text:?} generated text {generated_text:?}"
            ));
        }
    }
}

/// A value archived through one type and read as another of the same
/// shape.
macro_rules! same {
    ($value:expr, $type:ty) => {
        rkyv::from_bytes::<$type, Strict>(&rkyv::to_bytes::<Strict>($value).unwrap()).unwrap()
    };
}

/// The generated output of the hand output's name, its payload carried
/// across by its archive (each payload type is proven one above).
fn by_name(output: &hand::Response) -> generated::Output {
    match output {
        hand::Response::Read(data) => generated::Output::Read(same!(data, generated::Read_Data)),
        hand::Response::Subscribed => generated::Output::Subscribed,
        hand::Response::Changed(data) => {
            generated::Output::Changed(same!(data, generated::Changed_Data))
        }
        hand::Response::Redeemed(reset) => {
            generated::Output::Redeemed(same!(reset, generated::Reset))
        }
        hand::Response::Refused(refused) => {
            generated::Output::Refused(same!(refused, generated::Refused))
        }
    }
}

/// The tag a 0.5.0 output had and the tag the same output has in 0.6.0,
/// where they differ.
fn moved_tag(output: &hand::Response) -> Option<(u8, u8)> {
    match output {
        hand::Response::Changed(_) => Some((2, 3)),
        hand::Response::Redeemed(_) => Some((3, 2)),
        _ => None,
    }
}

impl Proof {
    /// One output, archived through the hand type and through the generated
    /// variant of its name: the two archives are one but the moved tag, the
    /// generated archive reads back as the generated value, and the datom
    /// texts are one.
    fn prove_reordered(&mut self, hand: hand::Response) {
        self.cases += 1;
        let name = format!("Response→Output.{hand:?}");
        let generated = by_name(&hand);
        let hand_bytes = rkyv::to_bytes::<Strict>(&hand).unwrap();
        let generated_bytes = rkyv::to_bytes::<Strict>(&generated).unwrap();
        let differing: Vec<(usize, u8, u8)> = hand_bytes
            .iter()
            .zip(generated_bytes.iter())
            .enumerate()
            .filter(|(_, (hand_byte, generated_byte))| hand_byte != generated_byte)
            .map(|(at, (hand_byte, generated_byte))| (at, *hand_byte, *generated_byte))
            .collect();
        let expected = moved_tag(&hand);
        let as_expected = hand_bytes.len() == generated_bytes.len()
            && match expected {
                None => differing.is_empty(),
                Some((was, is)) => {
                    differing.len() == 1 && differing[0].1 == was && differing[0].2 == is
                }
            };
        if as_expected {
            if let Some((was, is)) = expected {
                eprintln!("{name}: the tag at byte {} moves {was} -> {is}, nothing else", differing[0].0);
            }
        } else {
            self.differences.push(format!(
                "{name}: expected {expected:?} as the one difference, found {differing:?} (hand {} bytes, generated {})",
                hand_bytes.len(),
                generated_bytes.len()
            ));
        }
        match rkyv::from_bytes::<generated::Output, Strict>(&generated_bytes) {
            Ok(back) if back == generated => {}
            other => self
                .differences
                .push(format!("{name}: the generated bytes read back as {other:?}")),
        }
        let (hand_text, generated_text) = (hand.text(), generated.text());
        if hand_text != generated_text {
            self.differences.push(format!(
                "{name}: hand text {hand_text:?} generated text {generated_text:?}"
            ));
        }
    }
}

/// A value's datom text, under the `datom` feature; nothing without it.
trait Datom {
    fn text(&self) -> Option<String>;
}

#[cfg(feature = "datom")]
impl<T: datom_codec::Datomizable> Datom for T {
    fn text(&self) -> Option<String> {
        use protos::{Compactable, Protosizable};
        Some(self.datomize(Vec::new()).protosize().compact())
    }
}

#[cfg(not(feature = "datom"))]
impl<T> Datom for T {
    fn text(&self) -> Option<String> {
        None
    }
}

fn window(name: hand::Name) -> hand::Window {
    hand::Window {
        name,
        used: hand::Percent(37),
        resets: hand::Instant(1_789_000_000),
        duration: hand::Minutes(10_080),
    }
}

fn reset(status: hand::ResetStatus) -> hand::Reset {
    hand::Reset {
        reset_id: hand::ResetId("reset-a1".to_string()),
        reset_status: status,
        expires: hand::Instant(1_789_100_000),
    }
}

fn reading(provider: hand::Provider, allowed: bool) -> hand::Reading {
    hand::Reading {
        provider,
        allowed,
        window_vector: vec![
            window(hand::Name::Session),
            window(hand::Name::Week),
            window(hand::Name::FableWeek),
        ],
        reset_vector: vec![
            reset(hand::ResetStatus::Available),
            reset(hand::ResetStatus::Redeemed),
        ],
        at: hand::Instant(1_788_999_000),
    }
}

fn address(aspect: signal_flow::Aspect, model: signal_flow::Model) -> hand::Address {
    hand::Address {
        aspect,
        topic: signal_flow::Topic("energyNexus".to_string()),
        model,
    }
}

fn caller() -> hand::Caller {
    hand::Caller {
        process_id: 4242,
        user_id: 1001,
    }
}

macro_rules! prove {
    ($proof:expr, $name:expr, $hand:ty, $generated:ty, $value:expr) => {
        <Proof as Proving<$hand, $generated>>::prove(&mut $proof, &$name, $value)
    };
}

#[test]
fn every_hand_type_archives_as_its_generated_type() {
    use signal_flow::{Aspect, Model};
    let mut proof = Proof {
        differences: Vec::new(),
        cases: 0,
    };
    for provider in [hand::Provider::Claude, hand::Provider::Codex] {
        prove!(proof, format!("Provider.{provider:?}"), hand::Provider, generated::Provider, provider);
    }
    prove!(proof, "Percent", hand::Percent, generated::Percent, hand::Percent(63));
    prove!(proof, "Instant", hand::Instant, generated::Instant, hand::Instant(1_789_000_000));
    prove!(proof, "Minutes", hand::Minutes, generated::Minutes, hand::Minutes(720));
    for name in [hand::Name::Session, hand::Name::Week, hand::Name::FableWeek] {
        prove!(proof, format!("Name.{name:?}"), hand::Name, generated::Name, name.clone());
        prove!(proof, format!("Window.{name:?}"), hand::Window, generated::Window, window(name));
    }
    prove!(proof, "ResetId", hand::ResetId, generated::ResetId, hand::ResetId("reset-a1".into()));
    for status in [
        hand::ResetStatus::Available,
        hand::ResetStatus::Redeeming,
        hand::ResetStatus::Redeemed,
    ] {
        prove!(proof, format!("ResetStatus.{status:?}"), hand::ResetStatus, generated::ResetStatus, status.clone());
        prove!(proof, format!("Reset.{status:?}"), hand::Reset, generated::Reset, reset(status));
    }
    for (provider, allowed) in [
        (hand::Provider::Claude, true),
        (hand::Provider::Codex, false),
    ] {
        prove!(proof, format!("Reading.{provider:?}"), hand::Reading, generated::Reading, reading(provider, allowed));
    }
    prove!(
        proof,
        "Reading.empty",
        hand::Reading,
        generated::Reading,
        hand::Reading {
            provider: hand::Provider::Claude,
            allowed: true,
            window_vector: Vec::new(),
            reset_vector: Vec::new(),
            at: hand::Instant(-1),
        }
    );
    for level in [
        hand::Level::Plenty,
        hand::Level::Fair,
        hand::Level::Low,
        hand::Level::Out,
    ] {
        prove!(proof, format!("Level.{level:?}"), hand::Level, generated::Level, level.clone());
        prove!(
            proof,
            format!("Threshold.{level:?}"),
            hand::Threshold,
            generated::Threshold,
            hand::Threshold {
                level,
                left: hand::Percent(40)
            }
        );
    }
    for aspect in [Aspect::Psyche, Aspect::Mind, Aspect::Field] {
        for model in [
            Model::Fable,
            Model::Opus,
            Model::Sonnet,
            Model::Haiku,
            Model::Astra,
            Model::Sol,
            Model::Luna,
            Model::Terra,
        ] {
            let name = format!("{aspect:?}.{model:?}");
            prove!(
                proof,
                format!("Recipient.{name}"),
                hand::Recipient,
                generated::Recipient,
                hand::Recipient(address(aspect.clone(), model.clone()))
            );
            prove!(
                proof,
                format!("Reminder.{name}"),
                hand::Reminder,
                generated::Reminder,
                hand::Reminder {
                    provider: hand::Provider::Codex,
                    level: hand::Level::Low,
                    recipient: hand::Recipient(address(aspect.clone(), model)),
                }
            );
        }
    }
    for standing in [hand::Standing::Over, hand::Standing::Under, hand::Standing::On] {
        prove!(proof, format!("Standing.{standing:?}"), hand::Standing, generated::Standing, standing.clone());
    }
    prove!(proof, "Caller", hand::Caller, generated::Caller, caller());
    let inputs = [
        hand::Query::Read(hand::Provider::Claude),
        hand::Query::Subscribe,
        hand::Query::Redeem(hand::Provider::Codex),
    ];
    for input in inputs.clone() {
        prove!(proof, format!("Query→Input.{input:?}"), hand::Query, generated::Input, input.clone());
        prove!(
            proof,
            format!("Call.{input:?}"),
            hand::Call,
            generated::Call,
            hand::Call {
                caller: caller(),
                input
            }
        );
    }
    let read = hand::Read_Data {
        reading: reading(hand::Provider::Claude, true),
        pace_vector: Vec::new(),
    };
    let changed = hand::Changed_Data {
        provider: hand::Provider::Codex,
        level: hand::Level::Out,
        reading: reading(hand::Provider::Codex, false),
    };
    prove!(proof, "Read_Data", hand::Read_Data, generated::Read_Data, read.clone());
    prove!(proof, "Changed_Data", hand::Changed_Data, generated::Changed_Data, changed.clone());
    let refusals = [
        hand::Refused::ProviderDown(hand::Provider::Claude),
        hand::Refused::NoReset,
        hand::Refused::RuleRefused("no reset is available".to_string()),
    ];
    for refused in refusals.clone() {
        prove!(proof, format!("Refused.{refused:?}"), hand::Refused, generated::Refused, refused);
    }
    let mut outputs = vec![
        hand::Response::Read(read),
        hand::Response::Subscribed,
        hand::Response::Changed(changed),
        hand::Response::Redeemed(reset(hand::ResetStatus::Redeemed)),
    ];
    outputs.extend(refusals.into_iter().map(hand::Response::Refused));
    for output in outputs {
        proof.prove_reordered(output);
    }
    assert!(
        proof.differences.is_empty(),
        "{} of {} cases differ:\n{}",
        proof.differences.len(),
        proof.cases,
        proof.differences.join("\n")
    );
    eprintln!(
        "{} cases: every hand archive is the generated archive, but the tags of Changed and Redeemed, swapped",
        proof.cases
    );
}

/// The digest is the hash of the authored Ethos text. It is the one part of
/// the wire that moves: a 0.5.0 peer is refused at its greeting.
#[test]
fn the_contract_digest_moves_with_the_ethos_text() {
    let hand = hand::Call::contract_digest();
    let generated = generated::Call::contract_digest();
    eprintln!("ContractDigest 0.5.0 {} generated {}", hand.0, generated.0);
    assert_ne!(hand.0, generated.0, "the 0.5.0 text and the generated contract's text are two texts");
    assert_eq!(hand.0, -301146476426041394, "the 0.5.0 digest, as tests/fixtures/contract.txt held it");
}
