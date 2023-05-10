//  2. Implementar un tipo de dato para puntos de una curva elíptica, junto con las
// operaciones de grupo (suma de puntos distintos y duplicación de puntos), utilizando
// la forma de Weierstrass. Hacer pruebas con la curva y**2 = x**3 -3x - 3 ... p=1021,
// determinando la cantidad de puntos que tiene la curva. Usando P=(379,1011), obtener kP, siendo k=655.

use std::ops::{Add, Sub, Div, Neg, Mul};

use num::{Zero, One, traits::Pow};

#[derive(Debug, Clone, Copy, Hash)]
pub struct PointElipticCurve <T> {
    x: Option<T>,
    y: Option<T>,
    a: T,
    b: T,
}


impl <T> PointElipticCurve <T> 
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
        + One
{
    pub fn new(x: T, y: T, a: T, b: T) -> Option<PointElipticCurve <T>> {      
        if y.pow(2) != (x.pow(3) + a * x + b) {
           return None;
        }

        Some(PointElipticCurve { x : Some(x), y : Some(y), a, b })
    }

    pub fn new_inf(a: T, b: T) -> PointElipticCurve <T> {
        PointElipticCurve { x: None, y: None, a, b }
    }
}


impl<T> PartialEq<(T, T, T, T)> for PointElipticCurve<T>
where
    T: PartialEq
        + std::ops::Add<T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Copy,
{
    // tuple (x, y, a, b)
    fn eq(&self, (x, y, a, b): &(T, T, T, T)) -> bool {
        match (self.x, self.y) {
            (None, None) => self.a == *a && self.b == *b,
            (Some(self_x), Some(self_y)) => return self_x == *x && self_y == *y && self.a == *a && self.b == *b,
            _ => return false,
        }
    }
}

// impl Eq for hashed a key como punto curva elpitica
impl<T> Eq for PointElipticCurve<T>
where
    T: PartialEq
        + std::ops::Add<T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Copy,
{
}

// impl eq!!
impl<T> PartialEq<(T, T)> for PointElipticCurve<T>
where
    T: PartialEq
        + std::ops::Add<T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Copy,
{
    // tuple (x, y)
    fn eq(&self, (x, y): &(T, T)) -> bool {
        match (self.x, self.y) {
            (None, None) => false,
            (Some(self_x), Some(self_y)) => return self_x == *x && self_y == *y,
            _ => return false,
        }
    }
}

impl<T> PartialEq for PointElipticCurve<T>
where
    T: PartialEq
        + std::ops::Add<T>
        + std::ops::Sub<Output = T>
        + std::ops::Neg<Output = T>
        + Copy,
{
    fn eq(&self, other: &PointElipticCurve<T>) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }
}

impl<T> Add for PointElipticCurve<T>
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
            return Some(PointElipticCurve::new_inf(self.a, self.b));
        }

        if self == other && self.y.unwrap() == T::zero() {
            return Some(PointElipticCurve::new_inf(self.a, self.b));
        }

        if self.x == other.x && self.y == other.y {
            // The two points are the same, so we need to double
            let s = (((self.x.unwrap().pow(2)) * 3_isize) + self.a) / (self.y.unwrap() * 2_isize);
            let x = s.pow(2) - (self.x.unwrap() * 2_isize);
            let y = (s * (self.x.unwrap() - x)) - self.y.unwrap();

            return PointElipticCurve::new(x, y, self.a, self.b);
        }

        let s = (self.y.unwrap() - other.y.unwrap()) / (self.x.unwrap() - other.x.unwrap());
        let x = s.pow(2) - self.x.unwrap() - other.x.unwrap();
        let y = (s * (other.x.unwrap() - x)) - other.y.unwrap();

        PointElipticCurve::new(x, y, self.a, self.b)
    }
}

impl<T> Mul<usize> for PointElipticCurve<T>
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

    /// Multiplicacion de un escalar entero usize por un punto de la curva eliptica
    /// Por ejemplo:
    /// (170, 142) + (170, 142) = 2 * (170, 142) 
    /// ```
    /// let p = PointElipticCurve::new(170, 142, 0, 7).unwrap();
    /// let p2 = p * 2;
    /// assert_eq!(p2, (170, 142) + (170, 142));
    /// ```
    /// 
    fn mul(self, other: usize) -> Self::Output {
        let mut result = PointElipticCurve::new_inf(self.a, self.b);

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

impl<T> Neg for PointElipticCurve<T>
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
        PointElipticCurve::new(self.x.unwrap(), -self.y.unwrap(), self.a, self.b)
    }
}

#[cfg(test)]
mod test_points_eliptic_curve {
    use std::{collections::HashMap, time::Instant};

    use rand::Rng;

    use crate::body_finity::BodyFinity;

    use super::*;

    #[test]
    fn test_01_creation() {
        let point1 = PointElipticCurve::new(-1, -1, 5, 7);
        let point2 = PointElipticCurve::new(-1, -2, 5, 7);
        assert!(point1.is_some()); 
        assert_eq!(point1.unwrap(), (-1, -1, 5, 7));

        assert!(point2.is_none()); 
    }

    #[test]
    fn test_02_creation_for_bf() {
        let p = 103;
        let x = BodyFinity::new(p, 17);
        let y = BodyFinity::new(p, 64);
        let a = BodyFinity::new(p, 0);
        let b = BodyFinity::new(p, 7);

        let point = PointElipticCurve::new(x, y, a, b);
        assert!(point.is_some());
    }

    #[test]
    fn test_03_add() {
        let point1 = PointElipticCurve::new(-1, -1, 5, 7).unwrap();
        let point2 = PointElipticCurve::new(-1, 1, 5, 7).unwrap();
        let point_inf = PointElipticCurve::new_inf(5, 7);

        assert_eq!(point1 + point2, Some(point_inf)); // P1 + P2 = I
        assert_eq!(point1 + point_inf, Some(point1)); // addition identity
        assert_eq!(point2 + point_inf, Some(point2)); // addition identity
        assert_eq!(point2, (- point1).unwrap()); // P2 = -P1
        assert_eq!(point1, (- point2).unwrap()); // P1 = -P2
    }

    #[test]
    fn test_04_add() {
        // http://www.christelbach.com/ECCalculator.aspx

        let p: i32 = 37;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 7);
        assert_eq!(a+b, BodyFinity::new(p, 7));
        
        let x1: BodyFinity<i32> = BodyFinity::new(p, 6);
        let y1: BodyFinity<i32> = BodyFinity::new(p, 1);
        let point1: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x1, y1, a, b).unwrap();
        
        let x2: BodyFinity<i32> = BodyFinity::new(p, 8);
        let y2: BodyFinity<i32> = BodyFinity::new(p, 1);
        let point2: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x2, y2, a, b).unwrap();

        let x3: BodyFinity<i32> = BodyFinity::new(p, 23);
        let y3: BodyFinity<i32> = BodyFinity::new(p, 36);
        let point3: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x3, y3, a, b).unwrap();

        let res = point1 + point2;
        assert_eq!(res, Some(point3)); // P1 + P2 = P3

    }

    #[test]
    fn test_05_add(){
        let p = 223;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 7);
        assert_eq!(a+b, BodyFinity::new(p, 7));
        
        let x1: BodyFinity<i32> = BodyFinity::new(p, 192);
        let y1: BodyFinity<i32> = BodyFinity::new(p, 105);
        let point1: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x1, y1, a, b).unwrap();
        
        let x2: BodyFinity<i32> = BodyFinity::new(p, 17);
        let y2: BodyFinity<i32> = BodyFinity::new(p, 56);
        let point2: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x2, y2, a, b).unwrap();

        let x3: BodyFinity<i32> = BodyFinity::new(p, 170);
        let y3: BodyFinity<i32> = BodyFinity::new(p, 142);
        let point3: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x3, y3, a, b).unwrap();

        let res = point1 + point2;
        assert_eq!(res, Some(point3)); // P1 + P2 = P3 
    }

    #[test]
    fn test_05_add_two_sim() {
        // http://www.christelbach.com/ECCalculator.aspx

        let p: i32 = 37;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 7);
        assert_eq!(a+b, BodyFinity::new(p, 7));
        
        let x1: BodyFinity<i32> = BodyFinity::new(p, 6);
        let y1: BodyFinity<i32> = BodyFinity::new(p, 1);
        let point1: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x1, y1, a, b).unwrap();
        
        let x3: BodyFinity<i32> = BodyFinity::new(p, 18);
        let y3: BodyFinity<i32> = BodyFinity::new(p, 17);
        let point3: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x3, y3, a, b).unwrap();

        let res = point1 + point1;
        assert_eq!(res, Some(point3)); // P1 + P1 = 2P1 = P3

    }


    #[test]
    fn test_07_scalar_mul_groups(){
        let p = 223;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 7);
        assert_eq!(a+b, BodyFinity::new(p, 7));
        
        let x: BodyFinity<i32> = BodyFinity::new(p, 47);
        let y: BodyFinity<i32> = BodyFinity::new(p, 71);
        let g: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x, y, a, b).unwrap();

        let n = 21; // Finity group, finity cyclic group: {G, 2G, ..., nG}. . . n is order of the group

        // loop of {G, 2G, ..., (n-1)G}
        for k in 1..n {
            let k = k as usize;
            let res = g * k;
            println!("{:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})", k, g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        }

        // nG: point at infinity is the additive identity or 0
        assert_eq!(g * n, PointElipticCurve::new_inf(a, b)); 

    }

    #[test]
    fn test_08_scalar_mul_groups(){
        let p = 223;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 7);
        assert_eq!(a+b, BodyFinity::new(p, 7));
        
        let x: BodyFinity<i32> = BodyFinity::new(p, 15);
        let y: BodyFinity<i32> = BodyFinity::new(p, 86);
        let g: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x, y, a, b).unwrap();

        let n = 7; // Finity group, finity cyclic group: {G, 2G, ..., nG}. . . n is order of the group

        // loop of {G, 2G, ..., (n-1)G}
        for k in 1..n {
            let k = k as usize;
            let res = g * k;
            println!("{:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})", k, g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        }

        // nG: point at infinity is the additive identity or 0
        assert_eq!(g * n, PointElipticCurve::new_inf(a, b)); 
    }

    #[test]
    fn test_09_ejercicio2(){
        // Implementar un tipo de dato para puntos de una curva elíptica, junto con las
        // operaciones de grupo (suma de puntos distintos y duplicación de puntos), utilizando
        // la forma de Weierstrass. Hacer pruebas con la curva y**2=x**3-3x-3 y p=1021,
        // determinando la cantidad de puntos que tiene la curva. Usando P=(379,1011),
        // obtener kP, siendo k=655

        let p = 1021;
        let a: BodyFinity<i32> = BodyFinity::new(p, -3);
        let b: BodyFinity<i32> = BodyFinity::new(p, -3);

        let x: BodyFinity<i32> = BodyFinity::new(p, 379);
        let y: BodyFinity<i32> = BodyFinity::new(p, 1011);
        let g: PointElipticCurve<BodyFinity<i32>> = PointElipticCurve::new(x, y, a, b).unwrap();

        let k = 655;

        let x_g_expected = BodyFinity::new(p, 388);
        let y_g_expected = BodyFinity::new(p, 60);
        let k_g_expected = PointElipticCurve::new(x_g_expected, y_g_expected, a, b).unwrap();

        let res = g * k;
        assert_eq!(res, k_g_expected);
        println!("{:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})", k, g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());


        // Hacer pruebas con la curva y**2=x**3-3x-3 y p=1021,
        // determinando la cantidad de puntos que tiene la curva.

        let p = 1021;
        let a: BodyFinity<i32> = BodyFinity::new(p, 1);
        let b: BodyFinity<i32> = BodyFinity::new(p, 1);
        
        let mut points = vec![];

        // Brute-force search
        for x in 0..p {
            let x_val = BodyFinity::new(p, x);
            let y_squared = x_val.pow(3) + a*x_val + b;
            for y in 0..p {
                let y_val = BodyFinity::new(p, y);
                if y_squared == y_val.pow(2) {
                    let point = PointElipticCurve::new(x_val, y_val, a, b).unwrap();
                    points.push(point);
                }
            }
        }
        println!("Cantidad de puntos: {:?} + 1 punto del infinito", points.len());
    }


    struct Agreement{
        p: i32,
        a: BodyFinity<i32>,
        b: BodyFinity<i32>,
        g: PointElipticCurve<BodyFinity<i32>>,
        order_g: usize,
        cuantity_points_curve: usize,
    }


    impl Agreement {
        fn new(p: i32, a: BodyFinity<i32>, b: BodyFinity<i32>, g: PointElipticCurve<BodyFinity<i32>>) -> Self {
            let mut order_g = 0;

            // Brute-force search
            for k in 1..p {
                let k = k as usize;
                let res = g * k;
                if res == PointElipticCurve::new_inf(a, b) {
                    order_g = k; // se incluye el punto del infinito en la cuenta. Es decir {G, 2G, ..., (n-1)G, nG} donde nG sera el punto del infinito
                    break;
                }
            }

            let mut cuantity_points_curve = 0;

            // Brute-force search
            for x in 0..p {
                let x_val = BodyFinity::new(p, x);
                let y_squared = x_val.pow(3) + a*x_val + b;
                for y in 0..p {
                    let y_val = BodyFinity::new(p, y);
                    if y_squared == y_val.pow(2) {
                        cuantity_points_curve += 1;
                    }
                }
            }



            Self {
                p,
                a,
                b,
                g,
                order_g,
                cuantity_points_curve,
            }
        }

        fn order_g(&self) -> usize {
            self.order_g
        }

        fn quantity_points_curve(&self) -> usize {
            self.cuantity_points_curve + 1 // sumando el punto del infinito
        }

        fn generate_public_key(&self, private_key: usize) -> PointElipticCurve<BodyFinity<i32>> {
            self.g * private_key
        }

        fn generate_shared_secret(&self, public_key: PointElipticCurve<BodyFinity<i32>>, private_key: usize) -> PointElipticCurve<BodyFinity<i32>> {
            public_key * private_key
        }


        // fn generate_shared_secret(&self, public_key: PointElipticCurve<BodyFinity<i32>>, private_key: i32) -> PointElipticCurve<BodyFinity<i32>> {
        //     public_key * private_key
        // }
    }

    #[test]
    fn test_10_ejercicio3_other(){
        let p = 43;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 6);

        let g1 = PointElipticCurve::new(BodyFinity::new(p, 13), BodyFinity::new(p, 15), a, b).unwrap();
        let g2 = PointElipticCurve::new(BodyFinity::new(p, 9), BodyFinity::new(p, 2), a, b).unwrap();

        let agreement_g1 = Agreement::new(p, a, b, g1);
        let agreement_g2 = Agreement::new(p, a, b, g2);
        println!("Orden de g1: {:?}", agreement_g1.order_g());
        println!("Orden de g2: {:?}", agreement_g2.order_g());
        println!("Cantidad de puntos de la curva: {:?}", agreement_g1.quantity_points_curve());

        // ALICE
        let private_key_alice = rand::thread_rng().gen_range(1..agreement_g1.order_g());
        let public_key_alice = agreement_g1.generate_public_key(private_key_alice);

        // BOB
        let private_key_bob = rand::thread_rng().gen_range(1..agreement_g1.order_g());
        let public_key_bob = agreement_g1.generate_public_key(private_key_bob);

        // ALICE recibe la clave publica de BOB
        let shared_secret_alice = agreement_g1.generate_shared_secret(public_key_bob, private_key_alice);

        // BOB recibe la clave publica de ALICE
        let shared_secret_bob = agreement_g1.generate_shared_secret(public_key_alice, private_key_bob);

        println!("[G1] SClave compartida secreta de ALICE: {:?}", shared_secret_alice);
        println!("[G1] Clave compartida secreta de BOB: {:?}", shared_secret_bob);
        assert_eq!(shared_secret_alice, shared_secret_bob);

        // idem con G2
        // ALICE
        let private_key_alice = rand::thread_rng().gen_range(1..agreement_g2.order_g());
        let public_key_alice = agreement_g1.generate_public_key(private_key_alice);

        // BOB
        let private_key_bob = rand::thread_rng().gen_range(1..agreement_g2.order_g());
        let public_key_bob = agreement_g1.generate_public_key(private_key_bob);

        // ALICE recibe la clave publica de BOB
        let shared_secret_alice = agreement_g2.generate_shared_secret(public_key_bob, private_key_alice);

        // BOB recibe la clave publica de ALICE
        let shared_secret_bob = agreement_g2.generate_shared_secret(public_key_alice, private_key_bob);

        println!("[G2] Clave compartida secreta de ALICE: {:?}", shared_secret_alice);
        println!("[G2] Clave compartida secreta de BOB: {:?}", shared_secret_bob);
        assert_eq!(shared_secret_alice, shared_secret_bob);

    }
    #[test]
    fn test_10_ejercicio3(){
        // Implementar un esquema básico de acuerdo de clave de Diffie-Hellman usando
        // curvas elípticas. Usar la curva con p=43, y**2=x**3+6 ... como generador g=(13,15). 
        // ¿Qué sucede si se emplea el punto g=(9,2)?

        let p = 43;
        let a: BodyFinity<i32> = BodyFinity::new(p, 0);
        let b: BodyFinity<i32> = BodyFinity::new(p, 6);
        let mut points = vec![];

        // Brute-force search
        for x in 0..p {
            let x_val = BodyFinity::new(p, x);
            let y_squared = x_val.pow(3) + a*x_val + b;
            for y in 0..p {
                let y_val = BodyFinity::new(p, y);
                if y_squared == y_val.pow(2) {
                    let point = PointElipticCurve::new(x_val, y_val, a, b).unwrap();
                    println!("Punto: (x: {:?}, y: {:?})", point.x.unwrap().as_value(), point.y.unwrap().as_value());
                    points.push(point);
                }
            }
        }

        println!("Cantidad de puntos en la curva dada: {:?} + 1 punto del infinito", points.len());
        println!("------------------------------------------------------------------------");
        
        let g1 = PointElipticCurve::new(BodyFinity::new(p, 13), BodyFinity::new(p, 15), a, b).unwrap();
        let g2 = PointElipticCurve::new(BodyFinity::new(p, 9), BodyFinity::new(p, 2), a, b).unwrap();
        println!("Punto G1: (x: {:?}, y: {:?})", g1.x.unwrap().as_value(), g1.y.unwrap().as_value());
        
        // loop of {G, 2G, ..., (n-1)G}
        for k in 1..p {
            let k = k as usize;
            let res = g1 * k;
            if res == PointElipticCurve::new_inf(a, b) {
                println!("  {:?} * (x: {:?}, y:{:?}) = INF", k, g1.x.unwrap().as_value(), g1.y.unwrap().as_value());
                break;
            }
            println!("  {:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})", k, g1.x.unwrap().as_value(), g1.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        }
        
        println!("------------------------------------------------------------------------");
        println!("Punto G2: (x: {:?}, y: {:?})", g2.x.unwrap().as_value(), g2.y.unwrap().as_value());
        for k in 1..p {
            let k = k as usize;
            let res = g2 * k;
            if res == PointElipticCurve::new_inf(a, b) {
                break;
            }
            println!("  {:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})", k, g2.x.unwrap().as_value(), g2.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        }
        
        // ALice y Bob se ponen de acuerdo en el punto G, parametros de la curva y p
        // 
        // Alice elije a, Bob elije b....de forma privada van a elegir esos valores. 
        // Alice envia G^a, Bob envia G^b... eso se envia publicamente! por la internet
        // 
        // Alice calcula (G^b)^a, Bob calcula (G^a)^b... ahi compartiran la misma clave secreta !!

        // Con G1 SERIA:
        println!("------------------------------------------------------------------------");
        println!("Punto G1: (x: {:?}, y: {:?})", g1.x.unwrap().as_value(), g1.y.unwrap().as_value());
        let alice_a_private = BodyFinity::new(p, 3);
        println!("alice_a_private: {:?}", alice_a_private);
        let bob_b_private = BodyFinity::new(p, 11);
        println!("bob_b_private: {:?}", bob_b_private);
        
        let alice_ga_public = g1 * alice_a_private.as_value() as usize;
        let bob_gb_public = g1 * bob_b_private.as_value() as usize;
        println!("alice_ga_public: (x: {:?}, y: {:?})", alice_ga_public.x.unwrap().as_value(), alice_ga_public.y.unwrap().as_value());
        println!("bob_gb_public: (x: {:?}, y: {:?})", bob_gb_public.x.unwrap().as_value(), bob_gb_public.y.unwrap().as_value());


        let bob_gab_shared_private = alice_ga_public * bob_b_private.as_value() as usize;
        let alice_gab_shared_private = bob_gb_public * alice_a_private.as_value() as usize;
        println!("alice_gab_shared_private: (x: {:?}, y: {:?})", alice_gab_shared_private.x.unwrap().as_value(), alice_gab_shared_private.y.unwrap().as_value());
        println!("bob_gab_shared_private: (x: {:?}, y: {:?})", bob_gab_shared_private.x.unwrap().as_value(), bob_gab_shared_private.y.unwrap().as_value());

        println!("------------------------------------------------------------------------");

        // Con G2 SERIA:
        println!("Punto G2: (x: {:?}, y: {:?})", g2.x.unwrap().as_value(), g2.y.unwrap().as_value());
        let alice_a_private = BodyFinity::new(p, 3);
        println!("alice_a_private: {:?}", alice_a_private);
        let bob_b_private = BodyFinity::new(p, 11);
        println!("bob_b_private: {:?}", bob_b_private);

        let alice_ga_public = g2 * alice_a_private.as_value() as usize;
        let bob_gb_public = g2 * bob_b_private.as_value() as usize;
        println!("alice_ga_public: (x: {:?}, y: {:?})", alice_ga_public.x.unwrap().as_value(), alice_ga_public.y.unwrap().as_value());
        println!("bob_gb_public: (x: {:?}, y: {:?})", bob_gb_public.x.unwrap().as_value(), bob_gb_public.y.unwrap().as_value());


        let bob_gab_shared_private = alice_ga_public * bob_b_private.as_value() as usize;
        let alice_gab_shared_private = bob_gb_public * alice_a_private.as_value() as usize;
        // println!("alice_gab_shared_private: {:?}", alice_gab_shared_private);
        // println!("bob_gab_shared_private: {:?}", bob_gab_shared_private);
        println!("alice_gab_shared_private: (x: {:?}, y: {:?})", alice_gab_shared_private.x.unwrap().as_value(), alice_gab_shared_private.y.unwrap().as_value());
        println!("bob_gab_shared_private: (x: {:?}, y: {:?})", bob_gab_shared_private.x.unwrap().as_value(), bob_gab_shared_private.y.unwrap().as_value());


    }


    #[test]
    fn test11_ejercicio4(){
        // Considerar la curva y**2=x**3+905x+100 definida sobre el cuerpo primo de orden 1021 y
        // el punto generador (1006,416). Desarrollar alguna estrategia que permita resolver el
        // problema del logaritmo discreto kP=(612,827)
        let p = 1021;
        let a: BodyFinity<i32> = BodyFinity::new(p, 905);
        let b: BodyFinity<i32> = BodyFinity::new(p, 100);
        let g = PointElipticCurve::new(BodyFinity::new(p, 1006), BodyFinity::new(p, 416), a, b).unwrap();
        println!("Punto G: (x: {:?}, y: {:?})", g.x.unwrap().as_value(), g.y.unwrap().as_value());
        println!("y**2=x**3+905x+100");
        println!("p: {:?}", p);

        // loop of {G, 2G, ..., (n-1)G}
        for k in 1..p {
            let k = k as usize;
            let res = g * k;
            if res == PointElipticCurve::new_inf(a, b) {
                // println!("  {:?} * (x: {:?}, y:{:?}) = INF", k, g1.x.unwrap().as_value(), g1.y.unwrap().as_value());
                println!(" Cantidad de puntos del grupo (G, 2G, ..., nG) sera n: {:?}", k);

                break;
            }
            // println!("  {:?} * (x: {:?}, y:{:?}) = (x: {:?}, y:{:?})", k, g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        }

        let k_p = PointElipticCurve::new(BodyFinity::new(p, 612), BodyFinity::new(p, 827), a, b).unwrap();

        let mut k = 0;
        let mut res = PointElipticCurve::new_inf(a, b);
        loop {
            k += 1;
            res = g * k;
            if res == k_p {
                break;
            }
        }

        println!("k * G = (x: {:?}, y: {:?})", res.x.unwrap().as_value(), res.y.unwrap().as_value());
        println!("k * (x: {:?}, y: {:?}) = (x: {:?}, y: {:?})", g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        println!("k = {:?}", k);
    }


    #[test]
    fn test11_ejercicio4_brute_force(){
        // Considerar la curva y**2=x**3+905x+100 definida sobre el cuerpo primo de orden 1021 y
        // el punto generador (1006,416). Desarrollar alguna estrategia que permita resolver el
        // problema del logaritmo discreto kP=(612,827)
        let p = 1021;
        let a: BodyFinity<i32> = BodyFinity::new(p, 905);
        let b: BodyFinity<i32> = BodyFinity::new(p, 100);
        let g = PointElipticCurve::new(BodyFinity::new(p, 1006), BodyFinity::new(p, 416), a, b).unwrap();
        println!("Punto G: (x: {:?}, y: {:?})", g.x.unwrap().as_value(), g.y.unwrap().as_value());
        println!("y**2=x**3+905x+100");
        println!("p: {:?}", p);

        let k_g = PointElipticCurve::new(BodyFinity::new(p, 612), BodyFinity::new(p, 827), a, b).unwrap();

        let mut k = 0;
        let mut res = PointElipticCurve::new_inf(a, b);
        loop {
            k += 1;
            res = g * k;
            if res == k_g {
                break;
            }
        }

        println!("k * G = (x: {:?}, y: {:?})", res.x.unwrap().as_value(), res.y.unwrap().as_value());
        println!("k * (x: {:?}, y: {:?}) = (x: {:?}, y: {:?})", g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        println!("k = {:?}", k);
        assert_eq!(g * k, k_g);
    }

    #[test]
    fn test11_ejercicio4_bsgs(){
        // Considerar la curva y**2=x**3+905x+100 definida sobre el cuerpo primo de orden 1021 y
        // el punto generador (1006,416). Desarrollar alguna estrategia que permita resolver el
        // problema del logaritmo discreto kP=(612,827)
        let p = 1021;
        let a: BodyFinity<i32> = BodyFinity::new(p, 905);
        let b: BodyFinity<i32> = BodyFinity::new(p, 100);
        let g = PointElipticCurve::new(BodyFinity::new(p, 1006), BodyFinity::new(p, 416), a, b).unwrap();
        println!("Punto G: (x: {:?}, y: {:?})", g.x.unwrap().as_value(), g.y.unwrap().as_value());
        println!("y**2=x**3+905x+100");
        println!("p: {:?}", p);

        let k_g = PointElipticCurve::new(BodyFinity::new(p, 612), BodyFinity::new(p, 827), a, b).unwrap();


        // calcular timepo calculo col nInstant
        let now_bsgs = Instant::now();
        // Tamaño del bloque
        let k = bsgs(p, a, b, g, k_g);

        let res = g * k as usize;
        println!("[ORIGINAL] k * G = (x: {:?}, y: {:?})", k_g.x.unwrap().as_value(), k_g.y.unwrap().as_value());
        println!("[HACKED] k * G = (x: {:?}, y: {:?})", res.x.unwrap().as_value(), res.y.unwrap().as_value());
        println!("k * (x: {:?}, y: {:?}) = (x: {:?}, y: {:?})", g.x.unwrap().as_value(), g.y.unwrap().as_value(), res.x.unwrap().as_value(), res.y.unwrap().as_value());
        println!("k = {:?}", k);

        assert_eq!(k_g, g * (k as usize));

        let res_now_bsgs = now_bsgs.elapsed();

        let now = Instant::now();
        test11_ejercicio4_brute_force();
        println!("Time elapsed force-brute {:?}", now.elapsed());
        println!("Time elapsed BSGS {:?}", res_now_bsgs);
    }

    fn bsgs(p: i32, a: BodyFinity<i32>, b: BodyFinity<i32>, g: PointElipticCurve<BodyFinity<i32>>, k_g: PointElipticCurve<BodyFinity<i32>>) -> i32 {
        let m = ((p)as f64).sqrt().ceil() as i32;

        // baby_steps
        let mut baby_steps: HashMap<PointElipticCurve<BodyFinity<i32>>, i32> = HashMap::new();
        let mut res = PointElipticCurve::new_inf(a, b);
        for b in 0..m {
            baby_steps.insert(res, b);
            res = g * (b as usize);
        }

        // giant_steps
        let mut giant_steps: HashMap<PointElipticCurve<BodyFinity<i32>>, i32> = HashMap::new();
        let mut k = 0;
        for a in 0..m {
            let res = (k_g + (- (g * ((a * m) as usize))).unwrap()).unwrap();
            giant_steps.insert(res, a);
            if baby_steps.contains_key(&res) {
                k = a * m + baby_steps.get(&res).unwrap() - 1;
                break;
            }
        }
        k
    }

}