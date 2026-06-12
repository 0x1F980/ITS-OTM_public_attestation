use its_otm_public_attestation::field_arith::FieldElement;
use its_otm_public_attestation::poly::Polynomial;
use its_otm_public_attestation::{verify_public_attestation, OtmChainSigner};

/// End-to-end M31 public attestation: sign, publish bundle, verify without signer secrets.
#[test]
fn test_public_attestation_m31_flow() {
    let master_root = FieldElement::new(5);
    let poly_backward = Polynomial::new([FieldElement::new(5), FieldElement::new(3)]);
    let mut signer = OtmChainSigner::<2>::new(
        master_root,
        poly_backward,
        (FieldElement::new(1), FieldElement::new(8)),
        FieldElement::new(99),
    );

    let bundle = signer.sign(
        FieldElement::new(42),
        FieldElement::new(7),
        FieldElement::new(11),
        FieldElement::new(13),
    );

    assert!(bool::from(verify_public_attestation(&bundle)));

    let bundle2 = signer.sign(
        FieldElement::new(100),
        FieldElement::new(3),
        FieldElement::new(17),
        FieldElement::new(19),
    );
    assert!(bool::from(verify_public_attestation(&bundle2)));
    assert_ne!(bundle.tag.value(), bundle2.tag.value());
}
