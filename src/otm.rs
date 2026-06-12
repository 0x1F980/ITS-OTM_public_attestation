use crate::field_arith::FieldElement;
use crate::poly::Polynomial;
use crate::trapdoor::lagrange_interpolate;
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
pub fn combine_sss_chains(sss_forward: FieldElement, sss_backward: FieldElement) -> FieldElement {
    sss_forward + sss_backward
}

#[inline]
pub fn derive_forward_secret(
    prev_backward_point: (FieldElement, FieldElement),
    prev_message: FieldElement,
) -> FieldElement {
    prev_backward_point.1 + prev_message
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

pub fn verify_forward_share<const K: usize>(
    poly_forward: &Polynomial<K>,
    message: FieldElement,
    forward_point: (FieldElement, FieldElement),
) -> Choice {
    let x_matches = forward_point.0.ct_eq(&message);
    let expected_y = poly_forward.evaluate(message);
    let y_matches = forward_point.1.ct_eq(&expected_y);
    x_matches & y_matches
}

pub fn verify_backward_share<const K: usize>(
    master_root: FieldElement,
    prev_points: &[(FieldElement, FieldElement)],
    new_point: (FieldElement, FieldElement),
) -> Choice {
    let mut points = [(FieldElement::zero(), FieldElement::zero()); K];
    points[0] = (FieldElement::zero(), master_root);
    for (i, pt) in points.iter_mut().enumerate().take(K).skip(1) {
        let idx = i - 1;
        if idx < prev_points.len() {
            *pt = prev_points[idx];
        }
    }

    let expected_y = lagrange_interpolate(&points, new_point.0);
    new_point.1.ct_eq(&expected_y)
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
