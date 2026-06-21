import Otm.OtmIntegrity

/-!
# ITS-OTM — Lean formal verification (Otm module tree)

WC-MAC integrity bound P(forge) ≤ 1/p over M31 (matches `its_otm_public_attestation` Rust).
-/

namespace Otm

theorem otm_certificate : otmIntegrity :=
  otm_integrity

end Otm
