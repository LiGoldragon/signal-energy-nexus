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
wire! { pub struct Pace { pub provider: Provider, pub name: Name, pub target: Target, pub expected: Percent, pub used: Percent, pub standing: Standing, pub per_day: Percent } }
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
            caller: Caller { process_id: 1, user_id: 2 },
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
}
