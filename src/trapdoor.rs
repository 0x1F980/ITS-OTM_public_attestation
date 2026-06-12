use crate::field_arith::FieldElement;
use subtle::{Choice, ConditionallySelectable};

/// Lagrange interpolation over Z_p to evaluate the unique degree-(n-1) polynomial at `x`.
pub fn lagrange_interpolate(points: &[(FieldElement, FieldElement)], x: FieldElement) -> FieldElement {
    let mut result = FieldElement::zero();
    let n = points.len();

    for i in 0..n {
        let mut numerator = FieldElement::one();
        let mut denominator = FieldElement::one();

        for j in 0..n {
            let is_different = Choice::from((i != j) as u8);
            let term_num = x - points[j].0;
            let term_den = points[i].0 - points[j].0;

            let num_factor = FieldElement::conditional_select(&FieldElement::one(), &term_num, is_different);
            let den_factor = FieldElement::conditional_select(&FieldElement::one(), &term_den, is_different);

            numerator = numerator * num_factor;
            denominator = denominator * den_factor;
        }

        let basis = numerator * denominator.invert();
        result = result + (points[i].1 * basis);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lagrange_interpolation() {
        let points = [
            (FieldElement::new(1), FieldElement::new(8)),
            (FieldElement::new(2), FieldElement::new(11)),
        ];
        assert_eq!(lagrange_interpolate(&points, FieldElement::new(0)).value(), 5);
        assert_eq!(lagrange_interpolate(&points, FieldElement::new(2)).value(), 11);
    }
}
