use closed_word_ordinary_proof::carry;
use datom_codec::{Actualizing, Budget, Datomizable, Potential};
use protos::{Compactable, Protosizable, ReaderBudget};
use signal_energy_nexus::{
    Allowed, At, Expires, Instant, Provider, Reading, Reset, ResetId, ResetStatus,
};
use worded::Worded;

fn main() {
    const WORDS: &str = "private reset Δ witness 20491";
    let original = Reading {
        provider: Provider::Codex,
        allowed: Allowed(true),
        window_vector: vec![],
        reset_vector: vec![Reset {
            reset_id: ResetId::word(WORDS.to_owned()),
            reset_status: ResetStatus::Available,
            expires: Expires(Instant(1_789_100_000)),
        }],
        at: At(Instant(1_788_999_000)),
    };

    let carried = carry(original.clone());
    assert_eq!(carried, original);
    let wire = carried.datomize(Vec::new()).protosize().compact();
    assert!(wire.contains(WORDS));
    let mut budget = Budget {
        remaining: 256,
        reader: ReaderBudget::try_from(256).unwrap(),
        depth: 0,
        maximum_depth: 32,
    };
    let restored: Reading = Potential::<Reading>::from(wire.as_str())
        .actualize(&mut budget)
        .expect("the generated Reading decodes its actual Datom text");

    assert_eq!(restored, original);
    let id: &ResetId = &restored.reset_vector[0].reset_id;
    assert_eq!(id.words(), WORDS);
    for debug in [format!("{restored:?}"), format!("{restored:#?}")] {
        assert!(!debug.contains(WORDS));
        assert!(debug.contains("ResetId(<redacted>)"));
    }
    println!("Reading equality, exact ResetId words, and nested Debug redaction verified.");
    println!("Datom wire: {wire}");
    println!("Nested Debug: {restored:?}");
}
