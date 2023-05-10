use std::collections::HashMap;

use crate::finite_body::finite_body::FiniteBody;

use super::point_elliptic_curve::PointEllipticCurve;

pub struct AlgorithmsDLP;

impl AlgorithmsDLP{
    // https://andrea.corbellini.name/2015/06/08/elliptic-curve-cryptography-breaking-security-and-a-comparison-with-rsa/
    pub fn bsgs(
        p: i32,
        a: FiniteBody<i32>,
        b: FiniteBody<i32>,
        g: PointEllipticCurve<FiniteBody<i32>>,
        k_g: PointEllipticCurve<FiniteBody<i32>>,
    ) -> i32 {
        let m = ((p) as f64).sqrt().ceil() as i32;

        // baby_steps
        let mut baby_steps: HashMap<PointEllipticCurve<FiniteBody<i32>>, i32> = HashMap::new();
        let mut res = PointEllipticCurve::new_inf(a, b);
        for b in 0..m {
            baby_steps.insert(res, b);
            res = g * (b as usize);
        }

        // giant_steps
        let mut giant_steps: HashMap<PointEllipticCurve<FiniteBody<i32>>, i32> = HashMap::new();
        let mut k = 0;
        for a in 0..m {
            let res = (k_g + (-(g * ((a * m) as usize))).unwrap()).unwrap();
            giant_steps.insert(res, a);
            if baby_steps.contains_key(&res) {
                k = a * m + baby_steps.get(&res).unwrap() - 1;
                break;
            }
        }
        k
    }

    pub fn brute_force(
        p: i32,
        a: FiniteBody<i32>,
        b: FiniteBody<i32>,
        g: PointEllipticCurve<FiniteBody<i32>>,
        k_g: PointEllipticCurve<FiniteBody<i32>>,
    ) -> i32 {
        let mut k = 0;
        loop {
            k += 1;
            let res = g * k;
            if res == k_g {
                break;
            }
        }

        k as i32
    }
}

#[cfg(test)]
mod test_algorithms_dlp {
    use std::time::Instant;

    use super::*;
    
    #[test]
    fn test1_ejercicio4() {
        let p = 1021;
        let a: FiniteBody<i32> = FiniteBody::new(p, 905);
        let b: FiniteBody<i32> = FiniteBody::new(p, 100);
        let g = PointEllipticCurve::new(FiniteBody::new(p, 1006), FiniteBody::new(p, 416), a, b)
            .unwrap();
    
        let k_g =
            PointEllipticCurve::new(FiniteBody::new(p, 612), FiniteBody::new(p, 827), a, b).unwrap();

        println!(
            "Punto G: (x: {:?}, y: {:?})",
            g.x.unwrap().as_value(),
            g.y.unwrap().as_value()
        );
        println!("y**2=x**3+905x+100");
        println!("p: {:?}", p);
        println!("k * G =  (x: {:?}, y: {:?})", k_g.x.unwrap().as_value(), k_g.y.unwrap().as_value());

        let now_brute_force = Instant::now();
        let k_brute_force = AlgorithmsDLP::brute_force(p, a, b, g, k_g);
        let res_brute_force = g * k_brute_force as usize;
        let time_brute_force = now_brute_force.elapsed().as_millis();


        let now_bsgs = Instant::now();
        let k_bsgs = AlgorithmsDLP::bsgs(p, a, b, g, k_g);
        let res_bsgs = g * k_bsgs as usize;
        let time_bsgs = now_bsgs.elapsed().as_millis();

        assert_eq!(k_brute_force, k_bsgs);

        println!("BRUTE FORCE");
        println!("  k: {:?}", k_brute_force);
        println!(
            "  kG: (x: {:?}, y: {:?})",
            res_brute_force.x.unwrap().as_value(),
            res_brute_force.y.unwrap().as_value()
        );
        println!("  Time elapsed: {:?} ms", time_brute_force);

        println!("BSGS");
        println!("  k: {:?}", k_bsgs);
        println!(
            "  kG: (x: {:?}, y: {:?})",
            res_bsgs.x.unwrap().as_value(),
            res_bsgs.y.unwrap().as_value()
        );
        println!("  Time elapsed: {:?} ms", time_bsgs);
    }
}