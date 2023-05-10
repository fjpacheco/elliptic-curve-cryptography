use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::traits::Pow;
use num::{One, Zero};

use super::finite_body::FiniteBody;

impl<T> Neg for FiniteBody<T>
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

    fn neg(self) -> Self::Output {
        Self::new(self.p, self.p - self.value)
    }
}

impl<T> Pow<usize> for FiniteBody<T>
where
    T: Add<T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<usize, Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Self;

    fn pow(self, rhs: usize) -> Self::Output {
        Self::new(self.p, self.value.pow(rhs))
    }
}

impl<T> One for FiniteBody<T>
where
    T: Add<T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<usize, Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    fn one() -> Self {
        Self::new(T::one() + T::one(), T::one())
    }
}

impl<T> Zero for FiniteBody<T>
where
    T: Add<T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<usize, Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    fn zero() -> Self {
        Self::new(T::one(), T::zero())
    }

    fn is_zero(&self) -> bool {
        self.value == T::zero()
    }
}

impl<T> PartialOrd for FiniteBody<T>
where
    T: Add<T>
        + Mul<Output = T>
        + Div<Output = T>
        + Pow<usize, Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> PartialEq for FiniteBody<T>
where
    T: PartialEq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy,
{
    fn eq(&self, other: &FiniteBody<T>) -> bool {
        self.value == other.value
    }
}

impl<T> PartialEq<T> for FiniteBody<T>
where
    T: PartialEq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy,
{
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}

// impl Eq
impl<T> Eq for FiniteBody<T> where
    T: Eq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy
{
}


#[cfg(test)]
mod test_others{
    use crate::finite_body::finite_body::FiniteBody;

   
    #[test]
    fn test_01_neg() {
        let p = 7.0;

        let a = FiniteBody::new(p, 3.0);

        let b = -a;

        assert_eq!(b, 4.0);
    }

    #[test]
    fn test_02_neg() {
        let p = 7.0;

        let a = FiniteBody::new(p, -5.0);
        assert_eq!(a, 2.0);

        let b = -a;

        assert_eq!(b, 5.0);
    }

    #[test]
    fn test_03_neg() {
        let p = 7.0;

        assert_eq!(FiniteBody::new(p, 0.0), 0.0);
        assert_eq!(FiniteBody::new(p, 1.0), 1.0);
        assert_eq!(FiniteBody::new(p, 2.0), 2.0);
        assert_eq!(FiniteBody::new(p, 3.0), 3.0);
        assert_eq!(FiniteBody::new(p, 4.0), 4.0);
        assert_eq!(FiniteBody::new(p, 5.0), 5.0);
        assert_eq!(FiniteBody::new(p, 6.0), 6.0);
        assert_eq!(FiniteBody::new(p, 7.0), 0.0);

        assert_eq!(FiniteBody::new(p, 0.0), 0.0);
        assert_eq!(FiniteBody::new(p, -0.0), 0.0);

        assert_eq!(FiniteBody::new(p, -1.0), 6.0);
        assert_eq!(FiniteBody::new(p, -2.0), 5.0);
        assert_eq!(FiniteBody::new(p, -3.0), 4.0);
        assert_eq!(FiniteBody::new(p, -4.0), 3.0);
        assert_eq!(FiniteBody::new(p, -5.0), 2.0);
        assert_eq!(FiniteBody::new(p, -6.0), 1.0);
        assert_eq!(FiniteBody::new(p, -7.0), 0.0);

        assert_eq!(FiniteBody::new(p, -8.0), 6.0);

        assert_eq!(FiniteBody::new(1021, -3), 1018);
    }

}