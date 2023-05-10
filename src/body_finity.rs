use std::cmp::{PartialEq, Ordering};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::traits::Pow;
use num::{One, Zero};

#[derive(Copy, Clone, Debug, Hash)]
pub struct BodyFinity<T> {
    p: T,
    value: T,
}

impl<T> BodyFinity<T>
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
        BodyFinity {
            p,
            value: (value % p + p) % p,
        }
    }

    fn extended_euclidean_algorithm(mut a: i64, mut b: i64) -> (i64, i64) {
        let (mut s, mut old_s) = (0, 1);
        let (mut r, mut old_r) = (b, a);
        while r != 0 {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_s, s) = (s, old_s - quotient * s);
        }
        (old_r, old_s)
    }
    
    fn mod_inverse(a: i64, p: i64) -> Option<i64> {
        let (gcd, x) = Self::extended_euclidean_algorithm(a, p);
        if gcd == 1 {
            Some((x % p + p) % p)
        } else {
            None
        }
    }

    fn inverse(&self) -> Option<Self> {
        // with algorithm extended euclidean
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

        Some(Self::new(self.p, acum.as_value()))
        
    }

    pub fn as_value(&self) -> T {
        self.value
    }

}

impl<T> PartialEq for BodyFinity<T>
where
    T: PartialEq
        + std::ops::Add<T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Copy,
{
    fn eq(&self, other: &BodyFinity<T>) -> bool {
        self.value == other.value
    }
}

impl<T> PartialEq<T> for BodyFinity<T>
where
    T: PartialEq
        + std::ops::Add<T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Copy,
{
    fn eq(&self, other: &T) -> bool {
        self.value == *other
    }
}

// impl Eq
impl<T> Eq for BodyFinity<T> where T: Eq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy {
    
}


impl<T> Add for BodyFinity<T>
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

impl<T> Sub for BodyFinity<T>
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

impl<T> Neg for BodyFinity<T>
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

impl<T> Mul for BodyFinity<T>
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

impl<T> Div for BodyFinity<T>
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

// impl Pow of crate num.. rhs is usize
impl<T> Pow<usize> for BodyFinity<T>
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

// impl One! xD
impl<T> One for BodyFinity<T>
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


impl<T> Mul<isize> for BodyFinity<T>
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

    fn mul(self, other: isize) -> Self::Output {
        let mut result = Self::new(self.p, T::zero());

        for _ in 0..other {
            result = result + self;
        }

        result
    }
}

// impl Zero! xD
impl <T> Zero for BodyFinity<T>
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


// implr partOrd
impl<T> PartialOrd for BodyFinity<T>
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

#[cfg(test)]
mod test_cuerpo_finito {
    use super::*;

    #[test]
    fn test_01_add() {
        let p = 7.0;

        let a = BodyFinity::new(p, 3.0);
        let b = BodyFinity::new(p, 6.0);

        let c = a + b;

        assert_eq!(c, 2.0);
    }

    #[test]
    fn test_02_sub() {
        let p = 7.0;

        let a = BodyFinity::new(p, 3.0);
        let b = BodyFinity::new(p, 5.0);

        let c = a - b;

        assert_eq!(c, 5.0);
    }

    #[test]
    fn test_03_neg() {
        let p = 7.0;

        let a = BodyFinity::new(p, 3.0);

        let b = -a;

        assert_eq!(b, 4.0);
    }

    #[test]
    fn test_04_neg() {
        let p = 7.0;

        let a = BodyFinity::new(p, -5.0);
        assert_eq!(a, 2.0);

        let b = -a;

        assert_eq!(b, 5.0);
    }

    #[test]
    fn test_05_neg() {
        let p = 7.0;

        assert_eq!(BodyFinity::new(p, 0.0), 0.0);
        assert_eq!(BodyFinity::new(p, 1.0), 1.0);
        assert_eq!(BodyFinity::new(p, 2.0), 2.0);
        assert_eq!(BodyFinity::new(p, 3.0), 3.0);
        assert_eq!(BodyFinity::new(p, 4.0), 4.0);
        assert_eq!(BodyFinity::new(p, 5.0), 5.0);
        assert_eq!(BodyFinity::new(p, 6.0), 6.0);
        assert_eq!(BodyFinity::new(p, 7.0), 0.0);

        assert_eq!(BodyFinity::new(p, 0.0), 0.0);
        assert_eq!(BodyFinity::new(p, -0.0), 0.0);

        assert_eq!(BodyFinity::new(p, -1.0), 6.0);
        assert_eq!(BodyFinity::new(p, -2.0), 5.0);
        assert_eq!(BodyFinity::new(p, -3.0), 4.0);
        assert_eq!(BodyFinity::new(p, -4.0), 3.0);
        assert_eq!(BodyFinity::new(p, -5.0), 2.0);
        assert_eq!(BodyFinity::new(p, -6.0), 1.0);
        assert_eq!(BodyFinity::new(p, -7.0), 0.0);

        assert_eq!(BodyFinity::new(p, -8.0), 6.0);

        assert_eq!(BodyFinity::new(1021, -3), 1018);
    }

    #[test]
    fn test_06_mul() {
        let p = 7.0;

        assert_eq!(BodyFinity::new(p, 0.0) * BodyFinity::new(p, 0.0), 0.0);
        assert_eq!(BodyFinity::new(p, 2.0) * BodyFinity::new(p, 5.0), 3.0);
        assert_eq!(BodyFinity::new(p, 2.0) * BodyFinity::new(p, 4.0), 1.0);
        assert_eq!(
            BodyFinity::new(p, 2.0) * BodyFinity::new(p, 4.0) * BodyFinity::new(p, 4.0),
            4.0
        );
    }

    #[test]
    fn test_07_mul() {
        let p = 12; // no primo
                    // en los reales a * b = 0, entonces a o b es 0.. aca es interesante ver que 3 * 4 = 0 en Z12 cuando p es no primo
        assert_eq!(BodyFinity::new(p, 3) * BodyFinity::new(p, 4), 0);

        let p = 13; // es primo
                    // con los primos, no existe ningún valor a o b distintos de cero en Zp que satisfagan la condición de a*b = 0 en Zp.
                    // a*b = 0 en zp con p primo, podria ser cuando a o b sea 0.
        assert_eq!(BodyFinity::new(p, 0) * BodyFinity::new(p, 6), 0);
        assert_eq!(BodyFinity::new(p, 6) * BodyFinity::new(p, 0), 0);
    }
 
    
    #[test]
    fn test_08_inv_mul() {
        let p = 7; 
        assert_eq!(BodyFinity::new(p, 2).inverse().unwrap(), 4);
        assert_eq!(BodyFinity::new(p, 5).inverse().unwrap(), 3);
        assert_eq!(BodyFinity::new(p, 3).inverse().unwrap(), 5);
        
        assert_eq!((BodyFinity::new(p, 5)/BodyFinity::new(p, 2)), 6);
        assert_eq!((BodyFinity::new(p, 2)/BodyFinity::new(p, 5)), 6);

        assert_eq!((BodyFinity::new(p, 5)/BodyFinity::new(p, 3)), 4);
        assert_eq!((BodyFinity::new(p, 0)/BodyFinity::new(p, 5)), 0);

    }

    #[test]
    fn test_09_scalar_mul() {
        let p = 7; 
        assert_eq!((BodyFinity::new(p, 2) * 2), 4);
        assert_eq!((BodyFinity::new(p, 2) * 3), 6);
        assert_eq!((BodyFinity::new(p, 2) * 4), 1);

    }

}
