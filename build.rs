//! The contract's Rust is generated from its Ethos by ethos-zero and
//! committed; every build regenerates it and refuses a committed file that
//! differs.
use ethos_zero::{Actualizing, File, Potential, WordFields};
use ethos_zero_legacy::{Actualizing as _, Generating as _};

fn main() {
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest"));
    for name in ["library", "signal"] {
        let source_path = format!("ethos/{name}.ethos");
        let generated_path = format!("src/generated/{name}.rs");
        println!("cargo:rerun-if-changed={source_path}");
        println!("cargo:rerun-if-changed={generated_path}");
        let source = std::fs::read_to_string(root.join(&source_path)).expect("source");
        // Only the vocabulary has String newtypes. Keep the existing Signal
        // generator and dispatch API until that cohort is migrated separately.
        let generated = if name == "library" {
            let file = Potential::<File>::from(source)
                .actualize()
                .unwrap_or_else(|_| panic!("read {source_path}"));
            file.generate_with_words(WordFields::Closed)
                .unwrap_or_else(|_| panic!("generate {source_path}"))
        } else {
            let file = ethos_zero_legacy::Potential::<ethos_zero_legacy::File>::from(source)
                .actualize()
                .unwrap_or_else(|_| panic!("read {source_path}"));
            file.generate()
                .unwrap_or_else(|_| panic!("generate {source_path}"))
        };
        assert_eq!(
            generated,
            std::fs::read_to_string(root.join(&generated_path)).expect("generated"),
            "{generated_path} is not what ethos-zero generates from {source_path}"
        );
    }
}
