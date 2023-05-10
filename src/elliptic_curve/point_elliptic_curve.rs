use std::ops::{Add, Div, Mul, Neg, Sub};

use num::{traits::Pow, One, Zero};

#[derive(Debug, Clone, Copy, Hash)]
pub struct PointEllipticCurve<T> {
    pub x: Option<T>,
    pub y: Option<T>,
    pub a: T,
    pub b: T,
}

impl<T> PointEllipticCurve<T>
where
    T: Add<T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Div<T>
        + Pow<usize, Output = T>
        + PartialOrd
        + Copy
        + Neg<Output = T>
        + Zero
        + One,
{
    pub fn new(x: T, y: T, a: T, b: T) -> Option<PointEllipticCurve<T>> {
        if y.pow(2) != (x.pow(3) + a * x + b) {
            return None;
        }

        Some(PointEllipticCurve {
            x: Some(x),
            y: Some(y),
            a,
            b,
        })
    }

    pub fn new_inf(a: T, b: T) -> PointEllipticCurve<T> {
        PointEllipticCurve {
            x: None,
            y: None,
            a,
            b,
        }
    }
}

impl<T> Add for PointEllipticCurve<T>
where
    T: Add<T>
        + Div<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<isize, Output = T>
        + Pow<usize, Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.a != other.a || self.b != other.b {
            return None;
        }

        if self.x.is_none() && self.y.is_none() {
            return Some(other); // self is point infinity or the additive identity.
        }

        if other.x.is_none() && other.y.is_none() {
            return Some(self); // other is point infinity or the additive identity.
        }

        // Handle the case where the two points are additive inverses (that is, they have the same
        // x but a different y, causing a vertical line). This should return the point at infinity
        if self.x == other.x && self.y != other.y {
            return Some(PointEllipticCurve::new_inf(self.a, self.b));
        }

        if self == other && self.y.unwrap() == T::zero() {
            return Some(PointEllipticCurve::new_inf(self.a, self.b));
        }

        if self.x == other.x && self.y == other.y {
            // The two points are the same, so we need to double
            let s = (((self.x.unwrap().pow(2)) * 3_isize) + self.a) / (self.y.unwrap() * 2_isize);
            let x = s.pow(2) - (self.x.unwrap() * 2_isize);
            let y = (s * (self.x.unwrap() - x)) - self.y.unwrap();

            return PointEllipticCurve::new(x, y, self.a, self.b);
        }

        let s = (self.y.unwrap() - other.y.unwrap()) / (self.x.unwrap() - other.x.unwrap());
        let x = s.pow(2) - self.x.unwrap() - other.x.unwrap();
        let y = (s * (other.x.unwrap() - x)) - other.y.unwrap();

        PointEllipticCurve::new(x, y, self.a, self.b)
    }
}

impl<T> Mul<usize> for PointEllipticCurve<T>
where
    T: Add<T>
        + Div<Output = T>
        + Sub<Output = T>
        + Mul<isize, Output = T>
        + Neg<Output = T>
        + Pow<usize, Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Self;

    /// Multiplicacion de un escalar entero usize por un punto de la curva eliptica. Esta operacion era necesaria
    /// para calcular la multiplicacion por 2 o por 3 en la formulas del ADD de los puntos de la curva eliptica.
    ///
    /// Por ejemplo:
    /// (-1, -1) + (-1, -1) = 2 * (-1, -1)
    /// ```
    /// use tp1::elliptic_curve::point_elliptic_curve::PointEllipticCurve;
    /// let p = PointEllipticCurve::new(-1, -1, 5, 7).unwrap();
    /// let p2 = p * 2;
    /// assert_eq!(p2, (p + p).unwrap());
    /// let p3 = p * 3;
    /// assert_eq!(p3, ((p + p).unwrap() + p).unwrap());
    /// ```
    ///
    fn mul(self, other: usize) -> Self::Output {
        let mut result = PointEllipticCurve::new_inf(self.a, self.b);

        for _ in 0..other {
            let partial = result + self;
            if partial.is_none() {
                panic!("The point is not on the curve");
            }
            result = partial.unwrap();
        }

        result
    }
}

impl<T> Neg for PointEllipticCurve<T>
where
    T: Add<T>
        + Div<T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Pow<usize, Output = T>
        + PartialOrd
        + Copy
        + Zero
        + One,
{
    type Output = Option<Self>;

    fn neg(self) -> Self::Output {
        if self.x.is_none() && self.y.is_none() {
            return Some(self);
        }
        PointEllipticCurve::new(self.x.unwrap(), -self.y.unwrap(), self.a, self.b)
    }
}

impl<T> PartialEq<(T, T, T, T)> for PointEllipticCurve<T>
where
    T: PartialEq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy,
{
    // tuple (x, y, a, b)
    fn eq(&self, (x, y, a, b): &(T, T, T, T)) -> bool {
        match (self.x, self.y) {
            (None, None) => self.a == *a && self.b == *b,
            (Some(self_x), Some(self_y)) => {
                self_x == *x && self_y == *y && self.a == *a && self.b == *b
            }
            _ => false,
        }
    }
}

impl<T> Eq for PointEllipticCurve<T> where
    T: PartialEq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy
{
}

impl<T> PartialEq<(T, T)> for PointEllipticCurve<T>
where
    T: PartialEq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy,
{
    // tuple (x, y)
    fn eq(&self, (x, y): &(T, T)) -> bool {
        match (self.x, self.y) {
            (None, None) => false,
            (Some(self_x), Some(self_y)) => self_x == *x && self_y == *y,
            _ => false,
        }
    }
}

impl<T> PartialEq for PointEllipticCurve<T>
where
    T: PartialEq + std::ops::Add<T> + std::ops::Sub<Output = T> + std::ops::Neg<Output = T> + Copy,
{
    fn eq(&self, other: &PointEllipticCurve<T>) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

#[cfg(test)]
mod test_points_eliptic_curve {
    use crate::finite_body::finite_body::FiniteBody;

    use super::*;

    #[test]
    fn test_01_creation() {
        let point1 = PointEllipticCurve::new(-1, -1, 5, 7);
        let point2 = PointEllipticCurve::new(-1, -2, 5, 7);
        assert!(point1.is_some());
        assert_eq!(point1.unwrap(), (-1, -1, 5, 7));

        assert!(point2.is_none());
    }

    #[test]
    fn test_02_creation_for_bf() {
        let p = 103;
        let x = FiniteBody::new(p, 17);
        let y = FiniteBody::new(p, 64);
        let a = FiniteBody::new(p, 0);
        let b = FiniteBody::new(p, 7);

        let point = PointEllipticCurve::new(x, y, a, b);
        assert!(point.is_some());
    }

    #[test]
    fn test_03_add() {
        let point1 = PointEllipticCurve::new(-1, -1, 5, 7).unwrap();
        let point2 = PointEllipticCurve::new(-1, 1, 5, 7).unwrap();
        let point_inf = PointEllipticCurve::new_inf(5, 7);

        assert_eq!(point1 + point2, Some(point_inf)); // P1 + P2 = I
        assert_eq!(point1 + point_inf, Some(point1)); // addition identity
        assert_eq!(point2 + point_inf, Some(point2)); // addition identity
        assert_eq!(point2, (-point1).unwrap()); // P2 = -P1
        assert_eq!(point1, (-point2).unwrap()); // P1 = -P2
    }

    #[test]
    fn test_04_add() {
        // http://www.christelbach.com/ECCalculator.aspx

        let p: i32 = 37;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 7);
        assert_eq!(a + b, FiniteBody::new(p, 7));

        let x1: FiniteBody<i32> = FiniteBody::new(p, 6);
        let y1: FiniteBody<i32> = FiniteBody::new(p, 1);
        let point1: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x1, y1, a, b).unwrap();

        let x2: FiniteBody<i32> = FiniteBody::new(p, 8);
        let y2: FiniteBody<i32> = FiniteBody::new(p, 1);
        let point2: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x2, y2, a, b).unwrap();

        let x3: FiniteBody<i32> = FiniteBody::new(p, 23);
        let y3: FiniteBody<i32> = FiniteBody::new(p, 36);
        let point3: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x3, y3, a, b).unwrap();

        let res = point1 + point2;
        assert_eq!(res, Some(point3)); // P1 + P2 = P3
    }

    #[test]
    fn test_05_add() {
        let p = 223;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 7);
        assert_eq!(a + b, FiniteBody::new(p, 7));

        let x1: FiniteBody<i32> = FiniteBody::new(p, 192);
        let y1: FiniteBody<i32> = FiniteBody::new(p, 105);
        let point1: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x1, y1, a, b).unwrap();

        let x2: FiniteBody<i32> = FiniteBody::new(p, 17);
        let y2: FiniteBody<i32> = FiniteBody::new(p, 56);
        let point2: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x2, y2, a, b).unwrap();

        let x3: FiniteBody<i32> = FiniteBody::new(p, 170);
        let y3: FiniteBody<i32> = FiniteBody::new(p, 142);
        let point3: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x3, y3, a, b).unwrap();

        let res = point1 + point2;
        assert_eq!(res, Some(point3)); // P1 + P2 = P3
    }

    #[test]
    fn test_05_add_two_sim() {
        // http://www.christelbach.com/ECCalculator.aspx

        let p: i32 = 37;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 7);
        assert_eq!(a + b, FiniteBody::new(p, 7));

        let x1: FiniteBody<i32> = FiniteBody::new(p, 6);
        let y1: FiniteBody<i32> = FiniteBody::new(p, 1);
        let point1: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x1, y1, a, b).unwrap();

        let x3: FiniteBody<i32> = FiniteBody::new(p, 18);
        let y3: FiniteBody<i32> = FiniteBody::new(p, 17);
        let point3: PointEllipticCurve<FiniteBody<i32>> =
            PointEllipticCurve::new(x3, y3, a, b).unwrap();

        let res = point1 + point1;
        assert_eq!(res, Some(point3)); // P1 + P1 = 2P1 = P3
    }

    #[test]
    fn test_07_scalar_mul_groups() {
        let p = 223;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 7);
        assert_eq!(a + b, FiniteBody::new(p, 7));

        let x: FiniteBody<i32> = FiniteBody::new(p, 47);
        let y: FiniteBody<i32> = FiniteBody::new(p, 71);
        let g: PointEllipticCurve<FiniteBody<i32>> = PointEllipticCurve::new(x, y, a, b).unwrap();

        let n = 21; // Finity group, finity cyclic group: {G, 2G, ..., nG}. . . n is order of the group

        // loop of {G, 2G, ..., (n-1)G}
        for k in 1..n {
            let k = k as usize;
            let res = g * k;
            println!(
                "{:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})",
                k,
                g.x.unwrap().as_value(),
                g.y.unwrap().as_value(),
                res.x.unwrap().as_value(),
                res.y.unwrap().as_value()
            );
        }

        // nG: point at infinity is the additive identity or 0
        assert_eq!(g * n, PointEllipticCurve::new_inf(a, b));
    }

    #[test]
    fn test_08_scalar_mul_groups() {
        let p = 223;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 7);
        assert_eq!(a + b, FiniteBody::new(p, 7));

        let x: FiniteBody<i32> = FiniteBody::new(p, 15);
        let y: FiniteBody<i32> = FiniteBody::new(p, 86);
        let g: PointEllipticCurve<FiniteBody<i32>> = PointEllipticCurve::new(x, y, a, b).unwrap();

        let n = 7; // Finity group, finity cyclic group: {G, 2G, ..., nG}. . . n is order of the group

        // loop of {G, 2G, ..., (n-1)G}
        for k in 1..n {
            let k = k as usize;
            let res = g * k;
            println!(
                "{:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})",
                k,
                g.x.unwrap().as_value(),
                g.y.unwrap().as_value(),
                res.x.unwrap().as_value(),
                res.y.unwrap().as_value()
            );
        }

        // nG: point at infinity is the additive identity or 0
        assert_eq!(g * n, PointEllipticCurve::new_inf(a, b));
    }

    #[test]
    fn test_09_ejercicio2() {
        // Implementar un tipo de dato para puntos de una curva elíptica, junto con las
        // operaciones de grupo (suma de puntos distintos y duplicación de puntos), utilizando
        // la forma de Weierstrass. Hacer pruebas con la curva y**2=x**3-3x-3 y p=1021,
        // determinando la cantidad de puntos que tiene la curva. Usando P=(379,1011),
        // obtener kP, siendo k=655

        let p = 1021;
        let a: FiniteBody<i32> = FiniteBody::new(p, -3);
        let b: FiniteBody<i32> = FiniteBody::new(p, -3);

        let x: FiniteBody<i32> = FiniteBody::new(p, 379);
        let y: FiniteBody<i32> = FiniteBody::new(p, 1011);
        let g: PointEllipticCurve<FiniteBody<i32>> = PointEllipticCurve::new(x, y, a, b).unwrap();

        let k = 655;

        // http://www.christelbach.com/ECCalculator.aspx
        let x_g_expected = FiniteBody::new(p, 388);
        let y_g_expected = FiniteBody::new(p, 60);
        let k_g_expected = PointEllipticCurve::new(x_g_expected, y_g_expected, a, b).unwrap();

        let res = g * k;
        assert_eq!(res, k_g_expected);
        println!(
            "kP = {:?} * (x: {:?}, y: {:?}) = (x: {:?}, y:{:?})",
            k,
            g.x.unwrap().as_value(),
            g.y.unwrap().as_value(),
            res.x.unwrap().as_value(),
            res.y.unwrap().as_value()
        );

        let mut points = vec![];

        // Brute-force search
        for x in 0..p {
            let x_val = FiniteBody::new(p, x);
            let y_squared = x_val.pow(3) + a * x_val + b;
            for y in 0..p {
                let y_val = FiniteBody::new(p, y);
                if y_squared == y_val.pow(2) {
                    let point = PointEllipticCurve::new(x_val, y_val, a, b).unwrap();
                    points.push(point);
                }
            }
        }
        println!(
            "Cantidad de puntos de la curva: {:?} + 1 punto del infinito",
            points.len()
        );
    }
}
