use std::ops::{Add, Div, Neg, Rem, Sub};

use num::{One, Zero};

use super::finite_body::FiniteBody;

impl<T> Sub for FiniteBody<T>
where
    T: Add<T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

#[cfg(test)]
mod test_sub {
    use crate::finite_body::finite_body::FiniteBody;

    #[test]
    fn test_01_sub_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, 3.0);
        let b = FiniteBody::new(p, 6.0);

        let c = a - b;

        assert_eq!(c, 4.0);
    }

    #[test]
    fn test_02_sub_ok() {
        let p = 1021;

        let a = FiniteBody::new(p, 330);
        let b = FiniteBody::new(p, 660);

        let c = a - b;

        assert_eq!(c, 691);
    }

    #[test] // mayor a p
    fn test_03_sub_two_numbers_that_equal_p_return_less_than_p() {
        let p = 1021;

        let a = FiniteBody::new(p, 1000);
        let b = FiniteBody::new(p, 21);

        let c = a - b;

        assert_eq!(c, 979);
    }

    #[test]
    fn test_04_sub_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, 3.0);
        let b = FiniteBody::new(p, 5.0);

        let c = a - b;

        assert_eq!(c, 5.0);
    }

    #[test]
    fn test_05_sub_with_number_zero() {
        let p = 7.0;

        let a = FiniteBody::new(p, 3.0);
        let b = FiniteBody::new(p, 0.0);

        let c = a - b;

        assert_eq!(c, 3.0);
    }

    #[test]
    fn test_06_sub_with_number_zero() {
        let p = 7.0;

        let a = FiniteBody::new(p, 0.0);
        let b = FiniteBody::new(p, 3.0);

        let c = a - b;

        assert_eq!(c, 4.0);
    }
}
