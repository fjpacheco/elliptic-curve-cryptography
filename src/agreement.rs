use num::traits::Pow;

use crate::{
    elliptic_curve::point_elliptic_curve::PointEllipticCurve, finite_body::finite_body::FiniteBody,
};

pub struct Agreement {
    pub p: i32,
    pub a: FiniteBody<i32>,
    pub b: FiniteBody<i32>,
    pub g: PointEllipticCurve<FiniteBody<i32>>,
    pub order_g: usize,
    pub cuantity_points_curve: usize,
}

impl Agreement {
    pub fn new(
        p: i32,
        a: FiniteBody<i32>,
        b: FiniteBody<i32>,
        g: PointEllipticCurve<FiniteBody<i32>>,
    ) -> Self {
        let mut order_g = 0;

        // Brute-force search
        for k in 1..p {
            let k = k as usize;
            let res = g * k;
            if res == PointEllipticCurve::new_inf(a, b) {
                order_g = k; // se incluye el punto del infinito en la cuenta. Es decir {G, 2G, ..., (n-1)G, nG} donde nG sera el punto del infinito
                break;
            }
        }

        let mut cuantity_points_curve = 0;

        // Brute-force search
        for x in 0..p {
            let x_val = FiniteBody::new(p, x);
            let y_squared = x_val.pow(3) + a * x_val + b;
            for y in 0..p {
                let y_val = FiniteBody::new(p, y);
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

    pub fn order_g(&self) -> usize {
        self.order_g // ya incluye el punto del infinito
    }

    pub fn quantity_points_curve(&self) -> usize {
        self.cuantity_points_curve + 1 // sumando el punto del infinito
    }

    pub fn generate_public_key(&self, private_key: usize) -> PointEllipticCurve<FiniteBody<i32>> {
        self.g * private_key
    }

    pub fn generate_shared_secret(
        &self,
        public_key: PointEllipticCurve<FiniteBody<i32>>,
        private_key: usize,
    ) -> PointEllipticCurve<FiniteBody<i32>> {
        public_key * private_key
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_01_ejercicio3_g1() {
        let p = 43;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 6);

        let g1 =
            PointEllipticCurve::new(FiniteBody::new(p, 13), FiniteBody::new(p, 15), a, b).unwrap();

        let agreement_g1 = Agreement::new(p, a, b, g1);
        println!("Orden de [G1]: {:?}", agreement_g1.order_g());
        println!(
            "Cantidad de puntos de la curva: {:?}",
            agreement_g1.quantity_points_curve()
        );

        // ALICE
        let private_key_alice = rand::thread_rng().gen_range(1..agreement_g1.order_g());
        // ALICE Enviara por internet la clave publica
        let public_key_alice = agreement_g1.generate_public_key(private_key_alice);
        
        // BOB
        let private_key_bob = rand::thread_rng().gen_range(1..agreement_g1.order_g());
        // BOB Enviara por internet la clave publica
        let public_key_bob = agreement_g1.generate_public_key(private_key_bob);

        // ALICE recibe la clave publica de BOB
        let shared_secret_alice =
            agreement_g1.generate_shared_secret(public_key_bob, private_key_alice);

        // BOB recibe la clave publica de ALICE
        let shared_secret_bob =
            agreement_g1.generate_shared_secret(public_key_alice, private_key_bob);

        println!(
            "[G1] Clave compartida secreta que tendrá ALICE: (x: {:?}, y: {:?})",
            shared_secret_alice.x.unwrap().as_value(),
            shared_secret_alice.y.unwrap().as_value()
        );
        println!(
            "[G1] Clave compartida secreta que tendrá BOB: (x: {:?}, y: {:?})",
            shared_secret_bob.x.unwrap().as_value(),
            shared_secret_bob.y.unwrap().as_value()
        );
        assert_eq!(shared_secret_alice, shared_secret_bob);
    }

    #[test]
    fn test_02_ejercicio3_g2() {
        let p = 43;
        let a: FiniteBody<i32> = FiniteBody::new(p, 0);
        let b: FiniteBody<i32> = FiniteBody::new(p, 6);

        let g2 =
            PointEllipticCurve::new(FiniteBody::new(p, 9), FiniteBody::new(p, 2), a, b).unwrap();

        let agreement_g2 = Agreement::new(p, a, b, g2);
        println!("Orden de [G2]: {:?}", agreement_g2.order_g());
        println!(
            "Cantidad de puntos de la curva: {:?}",
            agreement_g2.quantity_points_curve()
        );

        // ALICE
        let private_key_alice = rand::thread_rng().gen_range(1..agreement_g2.order_g());
        let public_key_alice = agreement_g2.generate_public_key(private_key_alice);

        // BOB
        let private_key_bob = rand::thread_rng().gen_range(1..agreement_g2.order_g());
        let public_key_bob = agreement_g2.generate_public_key(private_key_bob);

        // ALICE recibe la clave publica de BOB
        let shared_secret_alice =
            agreement_g2.generate_shared_secret(public_key_bob, private_key_alice);

        // BOB recibe la clave publica de ALICE
        let shared_secret_bob =
            agreement_g2.generate_shared_secret(public_key_alice, private_key_bob);

        println!(
            "[G2] Clave compartida secreta que tendrá ALICE: (x: {:?}, y: {:?})",
            shared_secret_alice.x.unwrap().as_value(),
            shared_secret_alice.y.unwrap().as_value()
        );
        println!(
            "[G2] Clave compartida secreta que tendrá BOB: (x: {:?}, y: {:?})",
            shared_secret_bob.x.unwrap().as_value(),
            shared_secret_bob.y.unwrap().as_value()
        );
        assert_eq!(shared_secret_alice, shared_secret_bob);
    }
}
