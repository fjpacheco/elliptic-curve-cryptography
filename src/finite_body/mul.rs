use std::cmp::{Ordering, PartialEq};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::traits::Pow;
use num::{One, Zero};

use super::finite_body::FiniteBody;

impl<T> Mul for FiniteBody<T>
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

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.p, self.value * other.value)
    }
}

impl<T> Mul<isize> for FiniteBody<T>
where
    T: Add<T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Pow<usize, Output = T>
        + Rem<Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Self;

    /// Multiplicacion por un escalar positivo, deberia ser USIZE pero no pude hacerlo funcionar y me quedo ISIZE.
    fn mul(self, other: isize) -> Self::Output {
        let mut result = Self::new(self.p, T::zero());

        for _ in 0..(other.abs() as usize) {
            result = result + self;
        }

        result

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_mul_ok() {
        let a = FiniteBody::new(7, 3);
        let b = FiniteBody::new(7, 5);
        let c = FiniteBody::new(7, 1);

        assert_eq!(a * b, c);
    }

    #[test]
    fn test_02_mul_ok() {
        let p = 7.0;

        assert_eq!(FiniteBody::new(p, 0.0) * FiniteBody::new(p, 0.0), 0.0);
        assert_eq!(FiniteBody::new(p, 2.0) * FiniteBody::new(p, 5.0), 3.0);
        assert_eq!(FiniteBody::new(p, 2.0) * FiniteBody::new(p, 4.0), 1.0);
        assert_eq!(
            FiniteBody::new(p, 2.0) * FiniteBody::new(p, 4.0) * FiniteBody::new(p, 4.0),
            4.0
        );
    }

    #[test]
    fn test_03_mul() {
        let p = 12; // no primo
                    // en los reales a * b = 0, entonces a o b es 0.. aca es interesante ver que 3 * 4 = 0 en Z12 cuando p es no primo
        assert_eq!(FiniteBody::new(p, 3) * FiniteBody::new(p, 4), 0);

        let p = 13; // es primo
                    // con los primos, no existe ningún valor a o b distintos de cero en Zp que satisfagan la condición de a*b = 0 en Zp.
                    // a*b = 0 en zp con p primo, podria ser cuando a o b sea 0.
        assert_eq!(FiniteBody::new(p, 0) * FiniteBody::new(p, 6), 0);
        assert_eq!(FiniteBody::new(p, 6) * FiniteBody::new(p, 0), 0);
    }

    #[test]
    fn test_04_scalar_mul() {
        let p = 7;
        assert_eq!((FiniteBody::new(p, 2) * 2), 4);
        assert_eq!((FiniteBody::new(p, 2) * 3), 6);
        assert_eq!((FiniteBody::new(p, 2) * 4), 1);
        assert_eq!((FiniteBody::new(p, 2) * -1), 5);
        assert_eq!((FiniteBody::new(p, 2) * -3), 1);
    }
}
