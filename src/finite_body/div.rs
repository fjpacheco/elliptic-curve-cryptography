use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::traits::Pow;
use num::{One, Zero};

use super::finite_body::FiniteBody;

impl<T> Div for FiniteBody<T>
where
    T: Add<T>
        + Mul<Output = T>
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

    fn div(self, other: Self) -> Self::Output {
        if other.value == T::zero() {
            panic!("Error division");
        }

        let inv = other.inverse();
        match inv {
            Some(inv) => self * inv,
            None => panic!("Error division"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_div() {
        let p = 7;
        assert_eq!(FiniteBody::new(p, 2).inverse().unwrap(), 4);
        assert_eq!(FiniteBody::new(p, 5).inverse().unwrap(), 3);
        assert_eq!(FiniteBody::new(p, 3).inverse().unwrap(), 5);

        assert_eq!((FiniteBody::new(p, 5) / FiniteBody::new(p, 2)), 6);
        assert_eq!((FiniteBody::new(p, 2) / FiniteBody::new(p, 5)), 6);

        assert_eq!((FiniteBody::new(p, 5) / FiniteBody::new(p, 3)), 4);
        assert_eq!((FiniteBody::new(p, 0) / FiniteBody::new(p, 5)), 0);

    }


}
