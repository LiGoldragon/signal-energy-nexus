#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    clippy::type_complexity
)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Refused {
    ProviderDown(crate::Provider),
    NoReset,
    NoCredit(crate::Provider),
    SpentUnavailable(crate::Provider),
    RuleRefused(String),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BalanceRefused {
    NoCredit(crate::Provider),
    SpentUnavailable(crate::Provider),
    ProviderDown(crate::Provider),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ProcessId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserId(pub i64);
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Caller {
    pub process_id: ProcessId,
    pub user_id: UserId,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Call {
    pub caller: Caller,
    pub input: Input,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Input {
    Read(crate::Provider),
    Subscribe,
    Redeem(crate::Provider),
    Balance(crate::Provider),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Read_Data {
    pub reading: crate::Reading,
    pub pace_vector: std::vec::Vec<crate::Pace>,
    pub turn_end_subscription: crate::TurnEndSubscription,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Changed_Data {
    pub provider: crate::Provider,
    pub level: crate::Level,
    pub reading: crate::Reading,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Output {
    Read(Read_Data),
    Subscribed,
    Redeemed(crate::Reset),
    Balance(crate::Balance),
    BalanceRefused(BalanceRefused),
    Changed(Changed_Data),
    Refused(Refused),
}
#[rustfmt::skip]
#[allow(clippy::result_large_err, clippy::ptr_arg, clippy::borrowed_box)]
pub trait Answering {
    fn read(&self, provider: &crate::Provider) -> std::result::Result<Read_Data, Output>;
    fn subscribe(&self) -> std::result::Result<(), Output>;
    fn redeem(
        &self,
        provider: &crate::Provider,
    ) -> std::result::Result<crate::Reset, Output>;
    fn balance(
        &self,
        provider: &crate::Provider,
    ) -> std::result::Result<crate::Balance, Output>;
}
#[rustfmt::skip]
pub trait Answerable {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output;
}
#[rustfmt::skip]
impl Answerable for Input {
    fn answer<Nexus: Answering>(&self, nexus: &Nexus) -> Output {
        match self {
            Input::Read(carried) => {
                nexus.read(carried).map_or_else(|refusal| refusal, Output::Read)
            }
            Input::Subscribe => {
                nexus.subscribe().map_or_else(|refusal| refusal, |()| Output::Subscribed)
            }
            Input::Redeem(carried) => {
                nexus.redeem(carried).map_or_else(|refusal| refusal, Output::Redeemed)
            }
            Input::Balance(carried) => {
                nexus.balance(carried).map_or_else(|refusal| refusal, Output::Balance)
            }
        }
    }
}
