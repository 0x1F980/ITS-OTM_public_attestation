#!/usr/bin/env bash
# Cross-repo pipeline: ITS-asymmetric encrypt → ITS-OTM sign → verify → decrypt
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ASSYM="${ITS_ASSYMETRIC_BIN:-$ROOT/../ITS-asymmetric/target/release/its_asymmetric}"
OTM="${ITS_OTM_BIN:-$ROOT/../ITS-OTM_public_attestation/target/release/its_otm}"

if [[ ! -x "$ASSYM" ]]; then
  echo "Build ITS-asymmetric first: cd ../ITS-asymmetric && cargo build --release --bin its_asymmetric" >&2
  exit 1
fi
if [[ ! -x "$OTM" ]]; then
  echo "Build ITS-OTM first: cargo build --release --bin its_otm" >&2
  exit 1
fi

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

"$ASSYM" keygen --out-dir "$TMP/keys"
"$OTM" keygen --out "$TMP/alice.state"

echo -n "cross-repo pipeline test" | "$ASSYM" encrypt \
  --pk "$TMP/keys/public.key" --in - --out "$TMP/msg.wire" 2>/dev/null

"$OTM" sign --state "$TMP/alice.state" --in "$TMP/msg.wire" --out "$TMP/msg.otm"
"$OTM" verify --bundle "$TMP/msg.otm" --payload "$TMP/msg.wire"

OUT=$("$ASSYM" decrypt \
  --pk "$TMP/keys/public.key" \
  --sk "$TMP/keys/secret.key" \
  --in "$TMP/msg.wire" --out - 2>/dev/null)

[[ "$OUT" == "cross-repo pipeline test" ]]
echo "OK: encrypt → OTM sign/verify → decrypt"
