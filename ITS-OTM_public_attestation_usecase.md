# ITS-OTM_public_attestation: Use-Cases & Forking Guide (ITS-OTM_public_attestation_usecase)

## License: GNU GPLv3 Only
## Target: Integrators, Application Developers & Fork Maintainers

> **Scope:** [ITS-OTM_SECURITY_LAYERS.md](ITS-OTM_SECURITY_LAYERS.md).


---

## 1. Public Audit of Sneakernet / AEH Payloads

Publish a `PublicAttestationBundle` alongside parasitic entropy injections. Any auditor verifies integrity without access to Alice's ratchet or Bob's trapdoor.

**ITS-routing integration:** AEH commands in `its-routing` attach OTM tags; optional `.otm` bundle files enable third-party verification.

---

## 2. Tunnel & Routing Header Integrity

`its_transport` modules `onion` and `tunnel` use `its_transport::otm::{generate_tag, verify_tag}` for onion headers and SCPST packets. For **public** post-hoc audit of a tunnel transmission, publish a bundle:

```rust
use its_otm_public_attestation::{
    create_public_attestation, verify_public_attestation, OtmChainSigner,
};

let mut signer = OtmChainSigner::<2>::new(
    master_root,
    poly_backward,
    initial_back_point,
    initial_msg,
);
let bundle = signer.sign(message, slope, k_mac, nonce);
// Publish bundle.to_otm_text() or serialize to JSON
assert!(bool::from(verify_public_attestation(&bundle)));
```

---

## 3. Third-Party Notary Verification

A notary receives only the `.otm` file:

```bash
its_otm verify --bundle attestation.otm && echo "Integrity OK"
```

No shared secrets required beyond what is **already in the published bundle**.

---

## 4. Forking Guide

1. Fork [ITS-OTM_public_attestation](https://github.com/0x1F464/ITS-OTM_public_attestation).
2. Preserve `#![no_std]` + constant-time discipline in field arithmetic.
3. If changing tag formula, update `verify_public_attestation` and all 6 documentation pillars.
4. Downstream crates pin git rev:

```toml
its_otm_public_attestation = { git = "ssh://git@github.com/YOUR_FORK/ITS-OTM_public_attestation.git", rev = "..." }
```

---

## 5. Relationship to Timelock & Core Secrecy

| Need | Use |
|---|---|
| Hide message content | `ITS` core (SCPST / OTP) |
| Delay release in time | `ITS-self_enclosed_timelock` |
| Prove message integrity publicly | **`ITS-OTM_public_attestation`** (this repo) |

These compose but remain **separate git repositories** by design.
