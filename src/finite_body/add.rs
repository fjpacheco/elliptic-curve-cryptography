use std::ops::{Add, Div, Neg, Rem, Sub};

use num::{One, Zero};

use super::finite_body::FiniteBody;

impl<T> Add for FiniteBody<T>
where
    T: Add<T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.p, self.value + other.value)
    }
}

#[cfg(test)]
mod test_add {
    use crate::finite_body::finite_body::FiniteBody;

    #[test]
    fn test_01_add_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, 3.0);
        let b = FiniteBody::new(p, 6.0);

        let c = a + b;

        assert_eq!(c, 2.0);
    }

    #[test]
    fn test_02_add_ok() {
        let p = 1021;

        let a = FiniteBody::new(p, 330);
        let b = FiniteBody::new(p, 660);

        let c = a + b;

        assert_eq!(c, 990);
    }

    #[test] // mayor a p
    fn test_03_add_two_numbers_that_equal_p_return_zero() {
        let p = 1021;

        let a = FiniteBody::new(p, 1000);
        let b = FiniteBody::new(p, 21);

        let c = a + b;

        assert_eq!(c, 0);
    }

    #[test]
    fn test_04_add_two_numbers_that_greater_than_p_return_one() {
        let p = 1021;

        let a = FiniteBody::new(p, 1000);
        let b = FiniteBody::new(p, 22);

        let c = a + b;

        assert_eq!(c, 1);
    }

    #[test]
    fn test_05_add_two_numbers_that_greater_than_p_return_two() {
        let p = 1021;

        let a = FiniteBody::new(p, 1000);
        let b = FiniteBody::new(p, 23);

        let c = a + b;

        assert_eq!(c, 2);
    }

    #[test]
    fn test_06_add_two_numbers_zero_return_zero() {
        let p = 1021;

        let a = FiniteBody::new(p, 0);
        let b = FiniteBody::new(p, 0);

        let c = a + b;

        assert_eq!(c, 0);
    }

    #[test]
    fn test_07_add_zero_with_number_return_number() {
        let p = 1021;

        let a = FiniteBody::new(p, 0);
        let b = FiniteBody::new(p, 23);

        let c = a + b;

        assert_eq!(c, 23);

        let a = FiniteBody::new(p, 23);
        let b = FiniteBody::new(p, 0);

        let c = a + b;

        assert_eq!(c, 23);
    }
}
