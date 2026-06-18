# ITS-OTM_public_attestation: Core Cryptographic Vision & Threat Model (ITS-OTM_public_attestation_vision)

## License: GNU GPLv3 Only
## Target: Cryptographers, Security Auditors, High-Assurance Architects

> **Scope:** [ITS-OTM_SECURITY_LAYERS.md](ITS-OTM_SECURITY_LAYERS.md) — WC-MAC attestation is **ITS**; no SHA256 on tag.


---

## 1. Introduction & Attestation Vision

Standard message authentication (HMAC-SHA256, Ed25519) assumes a computationally bounded adversary. Under an **omnipotent adversary (Eve)** with infinite computing power, computational MACs and signatures collapse.

`ITS-OTM_public_attestation` implements **Information-Theoretic Secrecy (ITS) integrity** using Wegman-Carter **One-Time MAC (OTM)** tags bound to **non-reproducible SSS forward/backward chains**. The design solves a critical operational paradox:

* **Old model:** If Alice shares `k_mac` and `nonce` with Bob *before* signing, Bob (or anyone who seizes his machine) can **forge** arbitrary new attestations.
* **New model:** One-time keys are **revealed only together with a single published attestation bundle**. Any third party can verify that bundle without knowing the signer's private chain state — but cannot forge future messages because the SSS chain step is consumed and non-reproducible.

This crate is a **standalone signing/attestation primitive**, distinct from SCPST payload secrecy (`ITS` core) and temporal delay (`ITS-self_enclosed_timelock`).

**Repository:** [https://github.com/0x1F980/ITS-OTM_public_attestation](https://github.com/0x1F980/ITS-OTM_public_attestation)

---

## 2. Threat Model: The Omnipotent Adversary (Eve)

```
                  +------------------------------------------+
                  |         EVE: Global Surveillance         |
                  |  (Records all bundles, infinite compute) |
                  +-----+------------------------------+-----+
                        |                              |
            +-----------v-----------+      +-----------v-----------+
            |  Signer (offline)     |      |  Public Verifiers     |
            |  Consumes chain step|      |  Bundle-only verify   |
            +-----------------------+      +-----------------------+
```

### Adversary Capabilities
1. **Universal passive recording** of every published attestation bundle.
2. **Infinite computing power**, including quantum arrays.
3. **Active tampering** of stored bundles (modify message, tag, or points).

### Security Guarantees
1. **Integrity (ITS):** Forging a valid tag on a modified message succeeds with probability $\le d/p$ (see mathematics document).
2. **Public verifiability:** Verification requires only the published `PublicAttestationBundle` — no ratchet, trapdoor, or private chain secrets.
3. **No pre-shared forging rights:** Revealed `(k_mac, nonce)` pairs are **one-time** and bound to a specific chain step; they cannot mint new attestations.

---

## 3. Structural Defenses

### Layer 1: Wegman-Carter One-Time MAC
$$ T = k_{\text{mac}} \cdot y + n \pmod p $$
where $y = y_{\text{forward}} + y_{\text{backward}}$ combines SSS chain evaluations.

### Layer 2: SSS Forward Integrity
The forward share $(m, y_{\text{forward}})$ must lie on the signer-published forward polynomial $P(x)$ at the attested message $m$.

### Layer 3: SSS Backward Chain Binding
The backward share must lie on the unique degree-$(K-1)$ polynomial defined by the public `master_root` at $x=0$ and the published previous backward point — proving the attestation consumed a valid, sequential chain step that **cannot be rewound or replayed**.

---

## 4. Relationship to the ITS Ecosystem

| Repository | Role |
|---|---|
| **`ITS` (core)** | SCPST secrecy, routing, tunnel transport — consumes `otm::*` via re-export |
| **`ITS-self_enclosed_timelock`** | Temporal delay + deniable time-lock puzzles |
| **`ITS-OTM_public_attestation` (this repo)** | Public integrity attestation with one-time key reveal |
| **`ITS-routing`** | CLI orchestration (AEH, sneakernet, routing) |

---

## 5. What This Crate Does *Not* Provide

* **Confidentiality** of the attested message (publish only what you intend to be public).
* **Anonymous authentication** (bundles may include identifiable chain coordinates).
* **Long-term signing keys** (each attestation is strictly one-time).
