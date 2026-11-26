//! Ordinary consumer: carry a Reading and compare a ResetId as a whole.
use signal_energy_nexus::{Reading, ResetId};

pub fn carry(reading: Reading) -> Reading {
    if let Some(reset) = reading.reset_vector.first() {
        let id: &ResetId = &reset.reset_id;
        assert_eq!(id, &id.clone());
    }
    reading
}
