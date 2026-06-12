# ITS-OTM_public_attestation: API Reference Manual & Hermetic Build Instructions (ITS-OTM_public_attestation_manual)

## License: GNU GPLv3 Only
## Target: Systems Software Engineers, Integrators & Security Auditors

---

## 1. Build & Test

```bash
git clone git@github.com:0x1F464/ITS-OTM_public_attestation.git
cd ITS-OTM_public_attestation
cargo test
cargo build --release --bin its_otm
nix-shell --run "cargo build --release --bin its_otm"   # optional
docker build -t its-otm:local .                         # optional musl image
```

M61 field:
```bash
cargo test --features m61
cargo check --features m61
```

---

## 2. Core Types

### `FieldElement` (`field_arith`)
Elements of $\mathbb{Z}_p$ (default M31; `m61` feature for M61).

* `FieldElement::new(u32) -> Self`
* `value(&self) -> FieldStorage`
* `Add`, `Sub`, `Mul`, `Neg`, `invert()`

### `Polynomial<const K>` (`poly`)
Fixed-degree polynomial for SSS evaluations.

* `Polynomial::new([FieldElement; K]) -> Self`
* `evaluate(x: FieldElement) -> FieldElement`

### `PublicAttestationBundle<const K>`
Published record verifiable without owner secrets:

| Field | Description |
|---|---|
| `message` | Attested field element $m$ |
| `forward_point` | $(m, y_f)$ forward SSS share |
| `backward_point` | $(x_b, y_b)$ backward SSS share |
| `tag` | Wegman-Carter tag $T$ |
| `k_mac_reveal` | One-time MAC key (revealed with this bundle only) |
| `nonce_reveal` | One-time nonce (revealed with this bundle only) |
| `master_root` | Public backward chain root $Q(0)$ |
| `prev_backward_points` | Previous backward point(s) for chain verify |
| `forward_poly` | Forward polynomial $P(x)$ |

### `OtmChainSigner<2>`
Signer-side chain state for sequential attestations.

---

## 3. OTM Primitives (`otm`)

### Linear Wegman-Carter tag
```rust
pub fn generate_tag(k_mac: FieldElement, y: FieldElement, nonce: FieldElement) -> FieldElement;
pub fn verify_tag(k_mac, y, nonce, tag) -> Choice;
```
Formula: $T = k_{\text{mac}} \cdot y + n \pmod p$

### Chained SSS tag
```rust
pub fn generate_chained_tag_with_points(
    forward_point, backward_point, k_mac, nonce
) -> FieldElement;

pub fn verify_chained_tag_with_points(...) -> Choice;
```
Uses $y = y_f + y_b$.

### Share verification
```rust
pub fn verify_forward_share<const K>(poly, message, forward_point) -> Choice;
pub fn verify_backward_share<const K>(master_root, prev_points, new_point) -> Choice;
pub fn derive_forward_secret(prev_backward_point, prev_message) -> FieldElement;
```

---

## 4. Public Attestation API (`public_attestation`)

```rust
pub fn create_public_attestation<const K>(
    message, forward_point, backward_point,
    k_mac, nonce, master_root,
    prev_backward_points, forward_poly,
) -> PublicAttestationBundle<K>;

pub fn verify_public_attestation<const K>(bundle: &PublicAttestationBundle<K>) -> Choice;
```

`OtmChainSigner::<2>::sign(message, slope, k_mac, nonce) -> PublicAttestationBundle<2>` advances the chain and returns a publishable bundle.

---

## 5. CLI (`its_otm`)

```bash
its_otm demo                    # print + verify deterministic M31 bundle
its_otm verify --bundle FILE    # verify .otm text bundle; exit 0 if VALID
```

### Bundle text format (`.otm`)
```
message: 42
forward_x: 42
forward_y: 401
backward_x: 3
backward_y: 14
tag: 4578
k_mac: 11
nonce: 13
master_root: 5
forward_c0: 107
forward_c1: 7
prev_backward_x: 1
prev_backward_y: 8
```

---

## 6. Integration (ITS-core / ITS-net)

Add to `Cargo.toml`:
```toml
its_otm_public_attestation = { git = "ssh://git@github.com/0x1F464/ITS-OTM_public_attestation.git" }
```

ITS-core re-exports: `pub use its_otm_public_attestation as otm;`

See [ITS-OTM_public_attestation_usecase.md](ITS-OTM_public_attestation_usecase.md) for copy-paste snippets.
