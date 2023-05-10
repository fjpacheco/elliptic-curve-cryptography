use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::traits::Pow;
use num::{One, Zero};

#[derive(Copy, Clone, Debug, Hash)]
pub struct FiniteBody<T> {
    pub p: T,
    pub value: T,
}

impl<T> FiniteBody<T>
where
    T: Add<T>
        + Sub<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Neg<Output = T>
        + Zero
        + One,
{
    pub fn new(p: T, value: T) -> Self {
        FiniteBody {
            p,
            value: (value % p + p) % p,
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        // with algorithm extended euclidean:
        // for (p, value) coprimes: MCD(p, value) == 1
        // let (mut s, mut old_s) = (T::zero(), T::one());
        // let (mut r, mut old_r) = (self.p, self.value);
        // while r != T::zero() {
        //     let quotient = old_r / r;
        //     (old_r, r) = (r, old_r - quotient * r);
        //     (old_s, s) = (s, old_s - quotient * s);
        // }

        // if old_r == T::one() { // MCD == 1
        //     Some(Self::new(self.p, old_s))
        // } else {
        //     None
        // }

        // with fermat little theorem
        // for p prime
        let mut acum = Self::new(self.p, T::one()); //= self.value.pow(self.p - 2);
        let mut p_menos_2 = self.p - T::one() - T::one();
        while p_menos_2 > T::zero() {
            acum = acum * (*self);
            p_menos_2 = p_menos_2 - T::one();
        }

        if acum.as_value() == T::zero() {
            None
        } else {
            Some(Self::new(self.p, acum.as_value()))
        }
    }

    pub fn as_value(&self) -> T {
        self.value
    }
}

#[cfg(test)]
mod test_finite_body {
    use super::*;

    #[test]
    fn test_01_new_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, 3.0);

        assert_eq!(a, 3.0);
    }

    #[test]
    fn test_02_new_with_number_greater_than_p_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, 8.0);

        assert_eq!(a, 1.0);
    }

    #[test]
    fn test_03_new_with_number_equal_to_p_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, 7.0);

        assert_eq!(a, 0.0);
    }

    #[test]
    fn test_04_new_with_number_less_than_zero_ok() {
        let p = 7.0;

        let a = FiniteBody::new(p, -1.0);

        assert_eq!(a, 6.0);
    }

    #[test]
    fn test_05_inverse() {
        let p = 7;
        assert_eq!(FiniteBody::new(p, 1).inverse().unwrap(), 1);
        assert_eq!(FiniteBody::new(p, 2).inverse().unwrap(), 4);
        assert_eq!(FiniteBody::new(p, 3).inverse().unwrap(), 5);
        assert_eq!(FiniteBody::new(p, 4).inverse().unwrap(), 2);
        assert_eq!(FiniteBody::new(p, 5).inverse().unwrap(), 3);
        assert_eq!(FiniteBody::new(p, 6).inverse().unwrap(), 6);
        assert!(FiniteBody::new(p, 0).inverse().is_none());
        assert!(FiniteBody::new(p, 7).inverse().is_none());
    }
}
