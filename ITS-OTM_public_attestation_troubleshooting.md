# ITS-OTM_public_attestation: Troubleshooting & Error Recovery (ITS-OTM_public_attestation_troubleshooting)

## License: GNU GPLv3 Only
## Target: Operators, Integrators & Security Auditors

> **Scope:** [ITS-OTM_SECURITY_LAYERS.md](ITS-OTM_SECURITY_LAYERS.md).


---

## 1. Verification Returns INVALID

| Symptom | Likely Cause | Recovery |
|---|---|---|
| `verify_public_attestation` fails | Tampered `message`, `tag`, or share coordinates | Re-download bundle from trusted source; compare hash |
| Forward check fails | Wrong `forward_c0`/`forward_c1` or `forward_y` | Recompute $P(m)$ from published polynomial |
| Backward check fails | Wrong `master_root` or `prev_backward_*` | Ensure prev point matches signer's chain state **before** this step |
| Tag check fails | Wrong `k_mac`/`nonce` or modified `y` | Keys are one-time; cannot re-derive without signer |

---

## 2. Modulus Mismatch

**Symptom:** Values look correct but verification fails.

**Cause:** Bundle created under M31 (`default`) but verifier compiled with `--features m61` (or vice versa).

**Fix:** Use the same feature flag on signer and all verifiers. Field modulus is `2147483647` (M31) or `2305843009213693951` (M61).

---

## 3. Reused One-Time Keys

**Symptom:** Two bundles share identical `(k_mac, nonce)`.

**Cause:** Signer bug or manual bundle duplication.

**Impact:** Integrity may still hold per bundle, but replay detection is violated. **Never reuse** one-time keys across messages.

**Fix:** Advance `OtmChainSigner` state; derive fresh keys per attestation.

---

## 4. Empty or Missing `prev_backward` Fields

**Symptom:** First attestation in a chain fails public verify.

**Cause:** Bundle missing `prev_backward_x/y` when chain requires the initial backward anchor.

**Fix:** First bundle must include the previous backward point (typically the signer's initial anchor, e.g. $(1, Q(1))$).

---

## 5. CLI Errors

| Error | Fix |
|---|---|
| `parse error: missing field` | Ensure all required `.otm` keys present (see manual §5) |
| `read error` | Check file path for `its_otm verify --bundle` |
| Non-zero exit from `demo` | Rebuild crate; run `cargo test` |

---

## 6. Build Failures

| Error | Fix |
|---|---|
| `git fetch` failed in downstream ITS-core | Enable `.cargo/config.toml` `git-fetch-with-cli = true` and SSH keys for GitHub |
| Docker musl build fails | Install `musl-tools`; use `rust:1.78.0-slim` base per Dockerfile |

---

## 7. Constant-Time Verification Discipline

Do **not** branch on secret values before combining checks. Use `verify_public_attestation` as-is; it combines forward, backward, and tag results with `Choice` bitwise AND.
