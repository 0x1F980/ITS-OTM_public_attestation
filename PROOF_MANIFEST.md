# ITS-OTM — Proof manifest (v1.0.0)

| Theorem / obligation | Module | Layer |
|----------------------|--------|-------|
| WC-MAC tag generation/verify | `otm.rs` | Attestation |
| SSS-bound public attestation | `lib.rs` | Public bundle |
| Chained tag with share points | `sss_chain` re-export | Chaining |

**Build:** `cargo test`  
**Formal detail:** [ITS-OTM_FORMAL_VERIFICATION.md](ITS-OTM_FORMAL_VERIFICATION.md)

**Must NOT own:** Wire OTP body, transport onion.
