use crate::field_arith::FieldElement;
pub use sss_chain::{
    combine_sss_chains, derive_forward_secret, verify_backward_share, verify_forward_share,
};
use subtle::{Choice, ConstantTimeEq};
use zeroize::Zeroize;

/// Generates a Wegman-Carter One-Time MAC tag: `T = (K_MAC * y + N) mod p`.
#[inline]
pub fn generate_tag(k_mac: FieldElement, y: FieldElement, nonce: FieldElement) -> FieldElement {
    (k_mac * y) + nonce
}

/// Verifies a Wegman-Carter One-Time MAC tag in constant-time.
#[inline]
pub fn verify_tag(
    k_mac: FieldElement,
    y: FieldElement,
    nonce: FieldElement,
    tag: FieldElement,
) -> Choice {
    let expected = generate_tag(k_mac, y, nonce);
    tag.ct_eq(&expected)
}

#[inline]
pub fn generate_chained_tag_with_points(
    forward_point: (FieldElement, FieldElement),
    backward_point: (FieldElement, FieldElement),
    k_mac: FieldElement,
    nonce: FieldElement,
) -> FieldElement {
    let mut y = combine_sss_chains(forward_point.1, backward_point.1);
    let tag = generate_tag(k_mac, y, nonce);
    y.zeroize();
    tag
}

#[inline]
pub fn verify_chained_tag_with_points(
    forward_point: (FieldElement, FieldElement),
    backward_point: (FieldElement, FieldElement),
    k_mac: FieldElement,
    nonce: FieldElement,
    tag: FieldElement,
) -> Choice {
    let expected = generate_chained_tag_with_points(forward_point, backward_point, k_mac, nonce);
    tag.ct_eq(&expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_otm_generation_and_verification() {
        let k_mac = FieldElement::new(5);
        let y = FieldElement::new(3);
        let nonce = FieldElement::new(10);
        let tag = generate_tag(k_mac, y, nonce);
        assert!(bool::from(verify_tag(k_mac, y, nonce, tag)));
    }

    #[test]
    fn test_chained_tag_with_points() {
        let forward_point = (FieldElement::new(3), FieldElement::new(12));
        let backward_point = (FieldElement::new(1), FieldElement::new(7));
        let k_mac = FieldElement::new(5);
        let nonce = FieldElement::new(10);
        let tag = generate_chained_tag_with_points(forward_point, backward_point, k_mac, nonce);
        assert!(bool::from(verify_chained_tag_with_points(
            forward_point, backward_point, k_mac, nonce, tag
        )));
    }
}
