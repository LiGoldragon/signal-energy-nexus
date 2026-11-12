//! Energy stage-one ordinary contract. Branch energy-stage-one-a9a8d6, owned by Mind research Sol.
pub mod library;
#[path = "signal.rs"]
mod contract;
pub use contract::*;

pub const ETHOS: &str = concat!(
    include_str!("../Cargo.toml"), "\n",
    include_str!("../library.ethos"), "\n",
    include_str!("../signal.ethos"),
);
impl signal::Contracted for Call {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
