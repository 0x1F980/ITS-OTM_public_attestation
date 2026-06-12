use crate::field_arith::FieldElement;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A polynomial over Z_p of degree at most K - 1.
#[derive(Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct Polynomial<const K: usize> {
    pub coeffs: [FieldElement; K],
}

impl<const K: usize> Polynomial<K> {
    #[inline]
    pub fn new(coeffs: [FieldElement; K]) -> Self {
        Polynomial { coeffs }
    }

    pub fn evaluate(&self, x: FieldElement) -> FieldElement {
        if K == 0 {
            return FieldElement::zero();
        }

        let mut result = self.coeffs[K - 1];
        for i in (0..K - 1).rev() {
            result = (result * x) + self.coeffs[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_evaluation() {
        let p = Polynomial::new([FieldElement::new(5), FieldElement::new(3)]);
        assert_eq!(p.evaluate(FieldElement::new(0)).value(), 5);
        assert_eq!(p.evaluate(FieldElement::new(2)).value(), 11);
    }
}
