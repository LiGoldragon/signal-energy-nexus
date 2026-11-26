# Closed-word proof

The ordinary fixture is a separate consumer and its only direct dependency is
the generated Energy Signal, with no default features. No translator or codec
dependency is added for the privacy probe. The paired probe compiles the same
ordinary source and a copy with exactly one attempted field access, against
the same Signal artifact and with the same compiler options.

The translator fixture imports the separate Worded trait, builds a Reading
with a ResetId, carries it through the same ordinary function, encodes it with
the generated Datom implementation, decodes it as a Reading and checks both
typed equality and the exact recovered words. Debug output for the nested
ResetId is redacted in either mode. The Datom output is intentionally
reversible; this is a Rust access boundary, not encryption of the wire.

Both fixtures link one Signal crate. Its build checks the generated vocabulary
against the sibling closed-word generator and the unchanged Signal dispatch
against ethos-zero 970459d100ddee5f62a76c05642ecfabefba6329. Both authored
schemas are unchanged; `Read_Data` carries the generated `crate::Reading`.
Only intrinsic String newtypes close in this prerequisite, not raw String
payloads such as `RuleRefused(String)`.

On the synchronized builder, run
`bash proof/closed-word/verify.sh "$(mktemp -d)"` from the Signal checkout.
The argument is the empty scratch directory that receives the build and
evidence. The runner never modifies the checkouts.
