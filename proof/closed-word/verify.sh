#!/usr/bin/env bash
# Run the ordinary pair and translator on the synchronized builder only.
set -euo pipefail
export LC_ALL=C
test "$(hostname -s):$(id -un)" = prometheus:li
proof=$(realpath "${1:?Pass an empty scratch directory}")
test -d "$proof"
test -z "$(ls -A "$proof")"
fixtures=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
signal=$(realpath "$fixtures/../..")
export CARGO_TARGET_DIR="$proof/target"
export CARGO_TERM_COLOR=never

core=(--manifest-path "$fixtures/core/Cargo.toml" --locked --offline --no-default-features)
translator=(--manifest-path "$fixtures/translator/Cargo.toml" --locked --offline --no-default-features)
cargo tree "${core[@]}" --edges normal --depth 1 --prefix none > "$proof/ordinary-direct.txt"
cargo tree "${core[@]}" --edges normal,features > "$proof/ordinary-tree.txt"
cargo tree "${core[@]}" --edges normal --prefix none > "$proof/ordinary-packages.txt"
mapfile -t direct < "$proof/ordinary-direct.txt"
test "${#direct[@]}" = 2
[[ "${direct[0]}" == closed-word-ordinary-proof\ * ]]
[[ "${direct[1]}" == signal-energy-nexus\ * ]]
if grep -Eq '^(datom-codec|protos) ' "$proof/ordinary-packages.txt"; then
    echo "Ordinary target has a text codec dependency." >&2
    exit 1
fi

cargo check "${core[@]}" --message-format=json -v > "$proof/ordinary-cargo.jsonl" 2> "$proof/ordinary-cargo.stderr"
artifact=$(python3 - "$proof/ordinary-cargo.jsonl" "$signal/Cargo.toml" <<'PY'
import json
import sys

with open(sys.argv[1]) as source:
    rows = [json.loads(line) for line in source]
artifacts = [
    row for row in rows
    if row["reason"] == "compiler-artifact"
    and row["target"]["name"] == "signal_energy_nexus"
    and row["manifest_path"] == sys.argv[2]
]
assert len(artifacts) == 1, artifacts
artifact = artifacts[0]
assert artifact["features"] == [], artifact["features"]
rmeta, = (name for name in artifact["filenames"] if name.endswith(".rmeta"))
print(rmeta)
PY
)
mkdir -p "$proof/positive" "$proof/negative" "$proof/rustc-out"
cp "$fixtures/core/src/lib.rs" "$proof/positive/lib.rs"
python3 - "$proof/positive/lib.rs" "$proof/negative/lib.rs" <<'PY'
from pathlib import Path
import sys

positive, negative = map(Path, sys.argv[1:])
text = positive.read_text()
line = "        let id: &ResetId = &reset.reset_id;\n"
assert text.count(line) == 1
negative.write_text(text.replace(line, line + "        let _words: &String = &reset.reset_id.0;\n"))
PY
diff -u --label ordinary-positive --label ordinary-negative "$proof/positive/lib.rs" "$proof/negative/lib.rs" > "$proof/ordinary-pair.diff" || test "$?" = 1
sha256sum "$artifact" "$proof/"{positive,negative}/lib.rs > "$proof/ordinary-pair.sha256"
args=(--crate-name closed_word_ordinary_proof --edition=2021 --crate-type=lib --emit=metadata
      --error-format=json --extern "signal_energy_nexus=$artifact"
      -L "dependency=$(dirname "$artifact")" --out-dir "$proof/rustc-out")
printf '%q ' rustc "${args[@]}" > "$proof/ordinary-rustc.args"
printf '\n' >> "$proof/ordinary-rustc.args"
rustc "${args[@]}" "$proof/positive/lib.rs" 2> "$proof/positive.rustc.jsonl"
if rustc "${args[@]}" "$proof/negative/lib.rs" 2> "$proof/negative.rustc.jsonl"; then
    echo "Direct access unexpectedly compiled." >&2
    exit 1
fi
python3 - "$proof/negative.rustc.jsonl" > "$proof/privacy-diagnostic.txt" <<'PY'
import json
import sys

with open(sys.argv[1]) as source:
    rows = [json.loads(line) for line in source]
errors = [d for d in rows if d["level"] == "error" and d.get("spans")]
assert len(errors) == 1, errors
error = errors[0]
assert error["code"]["code"] == "E0616", error
assert "field `0` of struct `ResetId` is private" == error["message"], error
assert not any(d["level"] == "error" and d is not error and
               not d["message"].startswith("aborting due to") for d in rows), rows
assert any(".reset_id.0" in line["text"] for span in error["spans"] if span["is_primary"]
           for line in span["text"]), error
print(error["rendered"], end="")
PY

cargo tree "${translator[@]}" --edges normal --depth 1 --prefix none > "$proof/translator-direct.txt"
cargo tree "${translator[@]}" --edges normal,features > "$proof/translator-tree.txt"
cargo run "${translator[@]}" --message-format=json -v > "$proof/translator-cargo.jsonl" 2> "$proof/translator-cargo.stderr"
python3 - "$proof/translator-cargo.jsonl" "$signal/Cargo.toml" > "$proof/translator-results.txt" <<'PY'
import json
import sys

with open(sys.argv[1]) as source:
    lines = source.readlines()
rows = [json.loads(line) for line in lines if line.startswith("{")]
artifacts = [
    row for row in rows
    if row["reason"] == "compiler-artifact"
    and row["target"]["name"] == "signal_energy_nexus"
    and row["manifest_path"] == sys.argv[2]
]
assert len(artifacts) == 1, artifacts
assert artifacts[0]["features"] == ["datom"], artifacts[0]["features"]
output = "".join(line for line in lines if not line.startswith("{"))
assert "Reading equality, exact ResetId words, and nested Debug redaction verified." in output
print(output, end="")
PY

printf '%s\n' 'Ordinary consumer and paired E0616 privacy probe passed with Signal features [].'
cat "$proof/privacy-diagnostic.txt" "$proof/translator-results.txt"
