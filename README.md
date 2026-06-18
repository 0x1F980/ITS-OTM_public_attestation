# ITS-OTM_public_attestation: Wegman-Carter One-Time MAC Public Attestation

## License: GNU GPLv3 Only
## Target: Cryptographers, Security Auditors, High-Assurance Systems Engineers

**Read first:** **[ITS-OTM_SECURITY_LAYERS.md](ITS-OTM_SECURITY_LAYERS.md)** — WC-MAC is ITS; **no SHA256 on attestation tags**. Ecosystem master: [ITS_SECURITY_LAYERS.md](https://github.com/0x1F464/ITS/blob/master/ITS_SECURITY_LAYERS.md).

Welcome to the **`0x1F464/ITS-OTM_public_attestation`** repository. This is a standalone `#![no_std]` Rust implementation of **Wegman-Carter One-Time MAC** tags bound to **non-reproducible SSS forward/backward chains**, enabling **public verification** without access to the signer's private state.

Field arithmetic and SSS chain helpers come from the shared **[SSS_CHAIN](https://github.com/0x1F980/SSS_CHAIN)** crate (`sss_chain` dependency).

One-time keys are revealed **only with** each published attestation bundle — verifiers cannot forge new messages.

**Repository:** [https://github.com/0x1F464/ITS-OTM_public_attestation](https://github.com/0x1F464/ITS-OTM_public_attestation)

```bash
git clone git@github.com:0x1F464/ITS-OTM_public_attestation.git
cd ITS-OTM_public_attestation
cargo test
cargo build --release --bin its_otm
its_otm demo
nix-shell --run "cargo build --release --bin its_otm"   # optional hermetic build
docker build -t its-otm:local .                         # optional static musl image
```

---

## The 7-Pillar High-Assurance Documentation Architecture

```
                  +----------------------------------------------+
                  |                  README.md                   |
                  |                (This Portal)                 |
                  +----------------------+-----------------------+
                                         |
                  +----------------------v-----------------------+
                  |   ITS-OTM_SECURITY_LAYERS.md (#0 — ITS scope)  |
                  +----------------------+-----------------------+
                                         |
         +-------------------------------+-------------------------------+
         |                               |                               |
+--------v--------+             +--------v--------+             +--------v--------+
|    Vision       |             |   Mathematics   |             |     Manual      |
|  (Core Design & |             | (Proofs & Worked|             | (API Reference &|
|  Threat Model)  |             |  M31/M61 Examples)|             |  Build Guides)  |
+--------+--------+             +--------+--------+             +--------+--------+
         |                               |                               |
         +-------------------------------+-------------------------------+
         |                               |                               |
+--------v--------+             +--------v--------+             +--------v--------+
| Troubleshooting |             |    Use-Cases    |             |    HEADS_UP     |
| (Error Auditing |             |  (Integrations  |             | (Physical Shield|
| & Recoveries)   |             |   & Fork Guide) |             |  & WWIII Guide) |
+-----------------+             +-----------------+             +-----------------+
```

0. **[Security Layers (ITS-OTM_SECURITY_LAYERS.md)](ITS-OTM_SECURITY_LAYERS.md):** WC-MAC ITS scope; no SHA256 on tag; key reveal timing.
1. **[Vision (ITS-OTM_public_attestation_vision.md)](ITS-OTM_public_attestation_vision.md):** Public attestation threat model; one-time key reveal vs pre-shared forging.
2. **[Mathematics (ITS-OTM_public_attestation_mathematics.md)](ITS-OTM_public_attestation_mathematics.md):** WC forgery bounds, SSS chain binding, worked M31 example ($T=4578$).
3. **[Manual (ITS-OTM_public_attestation_manual.md)](ITS-OTM_public_attestation_manual.md):** Rust API, `PublicAttestationBundle`, CLI, build pipeline.
4. **[Troubleshooting (ITS-OTM_public_attestation_troubleshooting.md)](ITS-OTM_public_attestation_troubleshooting.md):** Modulus mismatch, reused keys, bundle field errors.
5. **[Use-Cases (ITS-OTM_public_attestation_usecase.md)](ITS-OTM_public_attestation_usecase.md):** AEH/sneakernet audit, tunnel integration, fork guide.
6. **[HEADS_UP (ITS-OTM_public_attestation_HEADS_UP.md)](ITS-OTM_public_attestation_HEADS_UP.md):** RAM zeroization, publication timing, coercion notes.

---

## Ecosystem

| Repository | Role |
|---|---|
| [SSS_CHAIN](https://github.com/0x1F980/SSS_CHAIN) | Shared SSS field, epoch, link API |
| [ITS](https://github.com/0x1F464/ITS) | Core SCPST secrecy, routing, tunnel |
| [ITS-self_enclosed_timelock](https://github.com/0x1F464/ITS-self_enclosed_timelock) | Time-lock puzzles |
| **ITS-OTM_public_attestation** (this repo) | Public OTM attestation |
| [ITS-routing](https://github.com/0x1F464/ITS-routing) | CLI orchestration |

---

## Quick API

```rust
use its_otm_public_attestation::{OtmChainSigner, verify_public_attestation};

let mut signer = OtmChainSigner::<2>::new(master_root, poly_backward, initial_back, initial_msg);
let bundle = signer.sign(message, slope, k_mac, nonce);
assert!(bool::from(verify_public_attestation(&bundle)));
```

Formula: $T = k_{\text{mac}} \cdot (y_f + y_b) + n \pmod p$
