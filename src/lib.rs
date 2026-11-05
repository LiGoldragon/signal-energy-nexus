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
wire! { pub enum Name { Session, Week } }
wire! { pub struct Window { pub name: Name, pub used: Percent, pub resets: Instant, pub duration: Minutes } }
wire! { pub struct ResetId(pub String); }
wire! { pub enum ResetStatus { Available, Redeeming, Redeemed } }
wire! { pub struct Reset { pub reset_id: ResetId, pub reset_status: ResetStatus, pub expires: Instant } }
wire! { pub struct Reading { pub provider: Provider, pub allowed: bool, pub window_vector: Vec<Window>, pub reset_vector: Vec<Reset>, pub at: Instant } }
wire! { pub enum Level { Plenty, Fair, Low, Out } }
wire! { pub struct Threshold { pub level: Level, pub left: Percent } }
wire! { pub enum Aspect { Psyche, Mind, Field } }
wire! { pub struct Topic(pub String); }
wire! { pub enum Layer { Primary, Secondary } }
wire! { pub struct Address { pub aspect: Aspect, pub topic: Topic, pub layer: Layer } }
wire! { pub struct Recipient(pub Address); }
wire! { pub struct Reminder { pub provider: Provider, pub level: Level, pub recipient: Recipient } }
wire! { pub enum Query { Read(Provider), Subscribe, Redeem(Provider) } }
wire! { pub struct Changed_Data { pub provider: Provider, pub level: Level, pub reading: Reading } }
wire! { pub enum Refused { ProviderDown(Provider), NoReset, RuleRefused(String) } }
wire! { pub enum Response { Read(Reading), Subscribed, Changed(Changed_Data), Redeemed(Reset), Refused(Refused) } }
pub const ETHOS: &str = include_str!("../signal.ethos");
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
#[cfg(test)]
mod tests {
    use super::*;
    use signal::{ByteViewable, Restorable, Signal, Signalizable};
    #[test]
    fn validated_binary_round_trip() {
        let value = Query::Read(Provider::Codex);
        let bytes = value.signalize().unwrap();
        let restored = Signal::<Query>::from(bytes.bytes().to_vec())
            .restore()
            .unwrap();
        assert_eq!(restored, value);
    }
}
