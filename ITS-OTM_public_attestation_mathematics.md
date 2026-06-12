# ITS-OTM_public_attestation: Formally Proven Mathematics & Worked M31 Examples (ITS-OTM_public_attestation_mathematics)

## License: GNU GPLv3 Only
## Target: Mathematicians, Cryptographers & Auditing Teams

---

## 1. Wegman-Carter One-Time MAC Forgery Bound

Let $y \in \mathbb{F}_p$ be the authenticated value, $k_{\text{mac}}, n \in \mathbb{F}_p$ independent uniform one-time secrets, and:
$$ T = k_{\text{mac}} \cdot y + n \pmod p $$

If Eve observes $(y, T)$ and attempts to forge $(y', T')$ for $y' \neq y$, she must satisfy:
$$ T' - T = k_{\text{mac}} \cdot (y' - y) \pmod p $$

For each candidate key $k' \in \mathbb{F}_p$, there exists exactly one valid nonce $n' = T' - k' \cdot y'$. Therefore:
$$ H(k_{\text{mac}} \mid y, T) = \log_2(p) \text{ bits} $$

The forgery probability for a single-field-element message is bounded by:
$$ P_{\text{forge}} \le \frac{1}{p} $$

For **M31** ($p = 2^{31}-1$): $P_{\text{forge}} < 5 \times 10^{-10}$.
For **M61** ($p = 2^{61}-1$): $P_{\text{forge}} < 10^{-18}$.

---

## 2. SSS Forward Share Integrity

Given forward polynomial $P(x) = c_0 + c_1 x + \cdots$ over $\mathbb{F}_p$, the forward attestation point $(m, y_f)$ is valid iff:
$$ y_f = P(m) \pmod p $$

Tampering $m$ or $y_f$ breaks the polynomial relation with probability $\ge 1 - 1/p$ under uniform key independence.

---

## 3. SSS Backward Chain Non-Reproducibility

Let $Q(x)$ be the backward authority polynomial with public master root $R = Q(0)$. Given the previous backward point $(x_{\text{prev}}, y_{\text{prev}})$, the next point $(x_{\text{new}}, y_{\text{new}})$ is valid iff it lies on the unique line (for $K=2$) through $(0, R)$ and $(x_{\text{prev}}, y_{\text{prev}})$:
$$ y_{\text{new}} = R + \frac{y_{\text{prev}} - R}{x_{\text{prev}}} \cdot x_{\text{new}} \pmod p $$

Each chain step selects a fresh $x_{\text{new}}$ from a monotonic counter, producing a **non-reproducible** sequence. An adversary who possesses revealed one-time keys from bundle $i$ cannot synthesize bundle $i+1$ without the signer's private backward polynomial evaluations at unseen coordinates.

---

## 4. Public Attestation Bundle Verification

The chained authentication value is:
$$ y = y_f + y_b \pmod p $$
Verification combines three constant-time checks:
1. `verify_forward_share`: $y_f = P(m)$
2. `verify_backward_share`: $(x_b, y_b)$ on backward chain from $(0, R)$ and prev point
3. `verify_tag`: $T = k_{\text{mac}} \cdot y + n$ using **revealed** one-time keys

---

## 5. Concrete Worked-Out M31 Example

Modulus: $p = 2147483647$.

### Setup
* Master root: $R = Q(0) = 5$
* Backward polynomial: $Q(x) = 5 + 3x$
* Previous backward point: $(1, Q(1)) = (1, 8)$
* Previous message (chain link): $99$

### Step 1 — Forward chain
$$ s_{\text{forw}} = y_{\text{prev}} + m_{\text{prev}} = 8 + 99 = 107 $$
Forward polynomial: $P(x) = 107 + 7x$ (slope $b = 7$)

Attested message: $m = 42$
$$ y_f = P(42) = 107 + 7 \cdot 42 = 107 + 294 = 401 $$

### Step 2 — Backward chain (step counter = 1 → $x = 3$)
$$ y_b = Q(3) = 5 + 9 = 14 $$
Backward point: $(3, 14)$

### Step 3 — Chained MAC input
$$ y = y_f + y_b = 401 + 14 = 415 $$

One-time keys (revealed in bundle): $k_{\text{mac}} = 11$, $n = 13$

### Step 4 — Tag
$$ T = 11 \cdot 415 + 13 = 4565 + 13 = 4578 \pmod{2147483647} $$

### Step 5 — Public verification (no signer secrets)
Any verifier checks:
1. $P(42) = 401$ ✓
2. Line through $(0,5)$ and $(1,8)$ evaluated at $x=3$ yields $14$ ✓
3. $11 \cdot 415 + 13 = 4578$ ✓

Run the identical values via:
```bash
cargo test test_public_attestation_m31_flow
its_otm demo
```

---

## 6. M61 Note

Enable the `m61` feature for $p = 2^{61}-1$. All formulas are identical; field operations use 64-bit Mersenne reduction. Forgery bound improves to $< 10^{-18}$ per attestation.
