#![allow(non_camel_case_types)]
macro_rules! wire {
    ($item:item) => {
        #[derive(
            rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash,
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
wire! { pub struct Hours(pub i64); }
wire! { pub struct Hour(pub i64); }
wire! { pub enum Weekday { Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday } }
wire! { pub struct Usefulness(pub Percent); }
wire! { pub struct Baseline(pub Percent); }
wire! { pub struct Span { pub weekday_vector: Vec<Weekday>, pub from: Hour, pub to: Hour, pub usefulness: Usefulness } }
wire! { pub struct Override { pub from: Instant, pub to: Instant, pub usefulness: Usefulness } }
wire! { pub struct Pace { pub provider: Provider, pub name: Name, pub target: Target, pub expected: Percent, pub used: Percent, pub standing: Standing, pub hours_left: Hours, pub useful_left: Decimal, pub per_useful_hour: Decimal } }
/// A finite decimal: no `NaN`, no infinity, and no `-0.0` apart from `0.0`.
/// Ethos `Decimal`. The datom codec's own decimal is the text form; this one
/// is the wire's, so the contract carries no text dependency without the
/// `datom` feature. Its archive is one little-endian `f64`, and reading one
/// back re-enters `try_from`, so a non-finite archived float is refused.
#[derive(rkyv::Archive, rkyv::Serialize, Clone, Copy, Debug, Default)]
pub struct Decimal(f64);
/// A float that is not finite has no Decimal.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NotFinite(pub f64);
impl std::fmt::Display for NotFinite {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} is not a finite decimal", self.0)
    }
}
impl std::error::Error for NotFinite {}
impl TryFrom<f64> for Decimal {
    type Error = NotFinite;
    fn try_from(value: f64) -> Result<Self, NotFinite> {
        if value.is_finite() {
            Ok(Self(if value == 0.0 { 0.0 } else { value }))
        } else {
            Err(NotFinite(value))
        }
    }
}
/// A decimal yields the finite float it holds.
pub trait Decimating {
    fn float(&self) -> f64;
}
impl Decimating for Decimal {
    fn float(&self) -> f64 {
        self.0
    }
}
impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
/// Sound because the value is finite.
impl Eq for Decimal {}
impl std::hash::Hash for Decimal {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.0.to_bits().hash(hasher)
    }
}
impl<D> rkyv::Deserialize<Decimal, D> for ArchivedDecimal
where
    D: rkyv::rancor::Fallible + ?Sized,
    D::Error: rkyv::rancor::Source,
{
    fn deserialize(&self, _deserializer: &mut D) -> Result<Decimal, D::Error> {
        let float = self.0.to_native();
        Decimal::try_from(float).map_err(<D::Error as rkyv::rancor::Source>::new)
    }
}
#[cfg(feature = "datom")]
impl datom_codec::Datomizable for Decimal {
    fn datomize(&self, at: datom_codec::Path) -> datom_codec::Datom {
        datom_codec::Datomizable::datomize(
            &datom_codec::Decimal::try_from(self.0).expect("a Decimal is finite"),
            at,
        )
    }
}
#[cfg(feature = "datom")]
impl datom_codec::Composing for Decimal {
    fn compose(
        datom: &datom_codec::Datom,
        budget: &mut datom_codec::Budget,
    ) -> Result<Self, datom_codec::Error> {
        use datom_codec::Decimating;
        <datom_codec::Decimal as datom_codec::Composing>::compose(datom, budget)
            .map(|decimal| Self(decimal.float()))
    }
}
wire! { pub struct Caller { pub process_id: i64, pub user_id: i64 } }
wire! { pub enum Query { Read(Provider), Subscribe, Redeem(Provider) } }
wire! { pub struct Call { pub caller: Caller, pub input: Query } }
wire! { pub struct Read_Data { pub reading: Reading, pub pace_vector: Vec<Pace> } }
wire! { pub struct Changed_Data { pub provider: Provider, pub level: Level, pub reading: Reading } }
wire! { pub enum Refused { ProviderDown(Provider), NoReset, RuleRefused(String) } }
wire! { pub enum Response { Read(Read_Data), Subscribed, Changed(Changed_Data), Redeemed(Reset), Refused(Refused) } }
pub const ETHOS: &str = include_str!("../signal.ethos");
impl signal::Contracted for Call {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
#[cfg(test)]
mod tests {
    use super::*;
    use ethos_zero::{Actualizing, File, Potential, Validating};
    use signal::{ByteViewable, Restorable, Signal, Signalizable};
    #[test]
    fn validated_binary_round_trip() {
        let value = Call {
            caller: Caller {
                process_id: 1,
                user_id: 2,
            },
            input: Query::Read(Provider::Codex),
        };
        let bytes = value.signalize().unwrap();
        let restored = Signal::<Call>::from(bytes.bytes().to_vec())
            .restore()
            .unwrap();
        assert_eq!(restored, value);
    }

    #[test]
    fn ethos_declaration_checks() {
        let file = Potential::<File>::from(ETHOS).actualize().unwrap();
        file.validate().unwrap();
    }

    #[test]
    fn a_decimal_is_finite_and_its_zero_has_no_sign() {
        assert!(Decimal::try_from(f64::NAN).is_err());
        assert!(Decimal::try_from(f64::INFINITY).is_err());
        assert_eq!(Decimal::try_from(-0.0).unwrap().float().to_bits(), 0.0f64.to_bits());
    }

    #[test]
    fn an_archived_float_that_is_not_finite_is_refused() {
        let value = Decimal::try_from(1.5).unwrap();
        let mut bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap().to_vec();
        assert_eq!(bytes.len(), 8, "one little-endian f64");
        bytes.copy_from_slice(&f64::NAN.to_le_bytes());
        let mut aligned = rkyv::util::AlignedVec::<16>::new();
        aligned.extend_from_slice(&bytes);
        assert!(rkyv::from_bytes::<Decimal, rkyv::rancor::Error>(&aligned).is_err());
    }

    #[cfg(feature = "datom")]
    #[test]
    fn pace_reads_in_useful_hours_in_datom_text() {
        use datom_codec::{Actualizing, Budget, Datomizable, Potential};
        use protos::{Compactable, Protosizable, ReaderBudget};
        let value = Pace {
            provider: Provider::Codex,
            name: Name::Week,
            target: Target::Even,
            expected: Percent(25),
            used: Percent(35),
            standing: Standing::Over,
            hours_left: Hours(124),
            useful_left: Decimal::try_from(83.5).unwrap(),
            per_useful_hour: Decimal::try_from(0.78).unwrap(),
        };
        let text = value.datomize(Vec::new()).protosize().compact();
        assert_eq!(text, "{ Codex Week Even 25 35 Over 124 83.5 0.78 }");
        let mut budget = Budget {
            remaining: 256,
            reader: ReaderBudget::try_from(256).unwrap(),
            depth: 0,
            maximum_depth: 32,
        };
        let restored: Pace = Potential::<Pace>::from(text.as_str())
            .actualize(&mut budget)
            .unwrap();
        assert_eq!(restored, value);
        let whole = Decimal::try_from(32.0).unwrap();
        assert_eq!(whole.datomize(Vec::new()).protosize().compact(), "32.0");
    }

    #[cfg(feature = "datom")]
    #[test]
    fn a_span_and_an_override_read_as_the_vision_writes_them() {
        use datom_codec::{Actualizing, Budget, Potential};
        use protos::ReaderBudget;
        let mut budget = Budget {
            remaining: 256,
            reader: ReaderBudget::try_from(256).unwrap(),
            depth: 0,
            maximum_depth: 32,
        };
        let span: Span = Potential::<Span>::from("{ [ Saturday Sunday ] 9 13 50 }")
            .actualize(&mut budget)
            .unwrap();
        assert_eq!(
            span,
            Span {
                weekday_vector: vec![Weekday::Saturday, Weekday::Sunday],
                from: Hour(9),
                to: Hour(13),
                usefulness: Usefulness(Percent(50)),
            }
        );
        let stretch: Override = Potential::<Override>::from("{ 1794859200 1794866400 0 }")
            .actualize(&mut budget)
            .unwrap();
        assert_eq!(stretch.usefulness, Usefulness(Percent(0)));
    }

    fn reminder() -> Reminder {
        Reminder {
            provider: Provider::Codex,
            level: Level::Low,
            recipient: Recipient(Address {
                aspect: signal_flow::Aspect::Mind,
                topic: signal_flow::Topic("energyNexus".into()),
                model: signal_flow::Model::Sol,
            }),
        }
    }

    #[test]
    fn a_reminder_to_a_model_addressed_flow_round_trips_in_binary() {
        let value = reminder();
        let bytes = value.signalize().unwrap();
        let restored = Signal::<Reminder>::from(bytes.bytes().to_vec())
            .restore()
            .unwrap();
        assert_eq!(restored, value);
    }

    #[cfg(feature = "datom")]
    #[test]
    fn a_reminder_names_the_model_in_datom_text() {
        use datom_codec::{Actualizing, Budget, Datomizable, Potential};
        use protos::{Compactable, Protosizable, ReaderBudget};
        let value = reminder();
        let text = value.datomize(Vec::new()).protosize().compact();
        assert!(
            text.contains("Sol"),
            "the recipient carries its model: {text}"
        );
        let mut budget = Budget {
            remaining: 256,
            reader: ReaderBudget::try_from(256).unwrap(),
            depth: 0,
            maximum_depth: 32,
        };
        let restored: Reminder = Potential::<Reminder>::from(text.as_str())
            .actualize(&mut budget)
            .unwrap();
        assert_eq!(restored, value);
    }
}
