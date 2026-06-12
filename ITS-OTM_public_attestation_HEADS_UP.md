# ITS-OTM_public_attestation: Tactical Heads-Up Manual (ITS-OTM_public_attestation_HEADS_UP)

## License: GNU GPLv3 Only
## Target: High-Assurance Operators & Physical-Security Teams

---

## 1. One-Time Keys in RAM

`k_mac` and `nonce` must be generated from a TRNG or ratchet **immediately before** signing. Zeroize signer state after `OtmChainSigner::sign` publishes the bundle. Do not log raw key material.

---

## 2. Publication Timing

Reveal `(k_mac, nonce)` **only together** with the attested message in the public bundle. Early disclosure to verifiers grants **forge capability** for that chain epoch.

---

## 3. Bundle Storage

Treat published bundles like signed court exhibits: immutable once released. Any edit invalidates `verify_public_attestation`.

---

## 4. Side-Channel Hygiene

Run attestation on bare-metal or hardened kernels when signing high-value statements. Field arithmetic is constant-time; still avoid co-located untrusted code during signing.

---

## 5. WWIII / Censorship Scenario

Public bundles can be replicated across mirrors. Verifiers need only static files and `its_otm verify` — no online signer contact required.

---

## 6. Coercion Scenario

Published bundles prove **integrity**, not **authorship identity**. Signer can plausibly claim a bundle was synthesized by anyone who possessed the one-time reveal keys at publication time. Design disclosure policy accordingly.
