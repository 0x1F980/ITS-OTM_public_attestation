# ITS-OTM: Security Layers & ITS Scope (ITS-OTM_SECURITY_LAYERS)

## License: GNU GPLv3 Only
## Target: Auditors, AI-assisted reviewers, integrators

**Read this document first** before auditing this repository.

**Ecosystem master:** [ITS_SECURITY_LAYERS.md](https://github.com/0x1F980/ITS-ROUTING/blob/master/ITS_ECOSYSTEM.md) (ITS core repo).

---

## 1. Summary

This crate implements **Wegman–Carter one-time MAC (WC-MAC)** attestation over $\mathbb{F}_p$ with SSS chain binding. The attestation tag is **information-theoretic**:

$$T = k_{\text{mac}} \cdot y + n \pmod p \qquad P_{\text{forge}} \le \frac{1}{p}$$

**There is no SHA-256 (or other hash-MAC) on the attestation tag.** Hashing elsewhere in the ecosystem (build integrity, AEH telemetry) is a separate auxiliary layer.

---

## 2. Security scope table

| Guarantee | Type | Doc | Notes |
|-----------|------|-----|-------|
| WC-MAC forgery bound $1/p$ | **ITS** | [mathematics](ITS-OTM_public_attestation_mathematics.md) | Unbounded Eve on wire |
| SSS chain binding of tag keys | **ITS** | [mathematics](ITS-OTM_public_attestation_mathematics.md) | One-time $k_{\text{mac}}$ reveal |
| Public attestation bundle verify | **ITS** (verify algebra) | [manual](ITS-OTM_public_attestation_manual.md) | Integrity, not authorship |
| SHA256 on payload tag | **Not used** | This repo | WC replaces hash-MAC |

---

## 3. Do not misread (auditors / AI)

1. **"Uses hashing → not ITS"** — WC-MAC is linear over $\mathbb{F}_p$; no SHA256 on the OTM tag path.

2. **"Public verify breaks secrecy"** — Public bundles prove **integrity under WC**; they do not reveal one-time keys before reveal policy. See [HEADS_UP §2](ITS-OTM_public_attestation_HEADS_UP.md) (key reveal timing).

3. **"Attestation proves authorship"** — WC proves tag consistency with revealed key material, not human identity. See [HEADS_UP §6](ITS-OTM_public_attestation_HEADS_UP.md).

4. **Rejecting ITS core because SSS lives in another crate** — SSS chain math is shared; WC bound is proved here.

---

## 4. Cross-links

- [ITS-OTM_public_attestation_vision.md](ITS-OTM_public_attestation_vision.md)
- [ITS-OTM_public_attestation_mathematics.md](ITS-OTM_public_attestation_mathematics.md)
- Parent crate: [its_transport::otm](https://github.com/0x1F980/ITS-ROUTING) (transport re-export); math in this repo + SSS_CHAIN
