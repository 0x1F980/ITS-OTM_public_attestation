use alloc::vec::Vec;

use crate::field_arith::FieldElement;
use crate::otm::{
    derive_forward_secret, generate_chained_tag_with_points, verify_backward_share,
    verify_chained_tag_with_points, verify_forward_share,
};
use crate::poly::Polynomial;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A published attestation record verifiable by any third party without owner secrets.
///
/// One-time MAC keys (`k_mac_reveal`, `nonce_reveal`) are disclosed **together with**
/// this single attestation. They cannot be reused to forge new messages; the next
/// attestation requires a fresh, non-reproducible SSS chain step.
#[derive(Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct PublicAttestationBundle<const K: usize> {
    pub message: FieldElement,
    pub forward_point: (FieldElement, FieldElement),
    pub backward_point: (FieldElement, FieldElement),
    pub tag: FieldElement,
    pub k_mac_reveal: FieldElement,
    pub nonce_reveal: FieldElement,
    pub master_root: FieldElement,
    pub prev_backward_points: Vec<(FieldElement, FieldElement)>,
    pub forward_poly: Polynomial<K>,
}

/// Signer-side SSS chain state for sequential public attestations.
#[derive(Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct OtmChainSigner<const K: usize> {
    pub master_root: FieldElement,
    pub poly_backward: Polynomial<K>,
    pub prev_back_point: (FieldElement, FieldElement),
    pub prev_msg: FieldElement,
    pub step_counter: u64,
}

impl OtmChainSigner<2> {
    pub fn new(
        master_root: FieldElement,
        poly_backward: Polynomial<2>,
        initial_back_point: (FieldElement, FieldElement),
        initial_msg: FieldElement,
    ) -> Self {
        OtmChainSigner {
            master_root,
            poly_backward,
            prev_back_point: initial_back_point,
            prev_msg: initial_msg,
            step_counter: 1,
        }
    }

    /// Creates a public attestation bundle and advances the non-reproducible chain.
    pub fn sign(
        &mut self,
        message: FieldElement,
        slope: FieldElement,
        k_mac: FieldElement,
        nonce: FieldElement,
    ) -> PublicAttestationBundle<2> {
        let x_val = ((self.step_counter % 2147483645) + 2) as u32;
        let x_i = FieldElement::new(x_val);
        let y_back = self.poly_backward.evaluate(x_i);
        let backward_point = (x_i, y_back);

        let s_forw = derive_forward_secret(self.prev_back_point, self.prev_msg);
        let is_zero = slope.ct_eq(&FieldElement::zero());
        let b_i = FieldElement::conditional_select(&slope, &FieldElement::one(), is_zero);
        let forward_poly = Polynomial::new([s_forw, b_i]);
        let y_forw = forward_poly.evaluate(message);
        let forward_point = (message, y_forw);

        let bundle = create_public_attestation(
            message,
            forward_point,
            backward_point,
            k_mac,
            nonce,
            self.master_root,
            &[self.prev_back_point],
            forward_poly,
        );

        self.prev_back_point = backward_point;
        self.prev_msg = message;
        self.step_counter += 1;

        bundle
    }
}

/// Builds a public attestation bundle from explicit chain coordinates and one-time keys.
pub fn create_public_attestation<const K: usize>(
    message: FieldElement,
    forward_point: (FieldElement, FieldElement),
    backward_point: (FieldElement, FieldElement),
    k_mac: FieldElement,
    nonce: FieldElement,
    master_root: FieldElement,
    prev_backward_points: &[(FieldElement, FieldElement)],
    forward_poly: Polynomial<K>,
) -> PublicAttestationBundle<K> {
    let tag = generate_chained_tag_with_points(forward_point, backward_point, k_mac, nonce);
    PublicAttestationBundle {
        message,
        forward_point,
        backward_point,
        tag,
        k_mac_reveal: k_mac,
        nonce_reveal: nonce,
        master_root,
        prev_backward_points: prev_backward_points.to_vec(),
        forward_poly,
    }
}

/// Verifies a published attestation in constant-time using only public bundle fields.
pub fn verify_public_attestation<const K: usize>(bundle: &PublicAttestationBundle<K>) -> Choice {
    let fwd_ok = verify_forward_share(&bundle.forward_poly, bundle.message, bundle.forward_point);
    let bwd_ok = verify_backward_share::<K>(
        bundle.master_root,
        &bundle.prev_backward_points,
        bundle.backward_point,
    );
    let tag_ok = verify_chained_tag_with_points(
        bundle.forward_point,
        bundle.backward_point,
        bundle.k_mac_reveal,
        bundle.nonce_reveal,
        bundle.tag,
    );
    fwd_ok & bwd_ok & tag_ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_attestation_roundtrip() {
        let master_root = FieldElement::new(5);
        let poly_backward = Polynomial::new([FieldElement::new(5), FieldElement::new(3)]);
        let initial_back = (FieldElement::new(1), FieldElement::new(8));
        let mut signer = OtmChainSigner::<2>::new(
            master_root,
            poly_backward,
            initial_back,
            FieldElement::new(99),
        );

        let bundle = signer.sign(
            FieldElement::new(42),
            FieldElement::new(7),
            FieldElement::new(11),
            FieldElement::new(13),
        );
        assert!(bool::from(verify_public_attestation(&bundle)));
    }

    #[test]
    fn test_tampered_bundle_fails() {
        let master_root = FieldElement::new(5);
        let poly_backward = Polynomial::new([FieldElement::new(5), FieldElement::new(3)]);
        let mut signer = OtmChainSigner::<2>::new(
            master_root,
            poly_backward,
            (FieldElement::new(1), FieldElement::new(8)),
            FieldElement::new(99),
        );
        let mut bundle = signer.sign(
            FieldElement::new(42),
            FieldElement::new(7),
            FieldElement::new(11),
            FieldElement::new(13),
        );
        bundle.message = FieldElement::new(43);
        assert!(!bool::from(verify_public_attestation(&bundle)));
    }
}
