pub mod config;

pub mod rsa {
    pub mod prime_gen {
        use std::error::Error;
        use num_bigint::{BigUint, RandBigInt, ToBigUint};
        use num_traits::*;
        use rand::Rng;
        use crate::rsa::config::config::*;

        pub fn fast_Modular_exponent(mut a: BigUint, mut q: BigUint, n: BigUint) -> BigUint {
            let mut r: BigUint = One::one();
            while q != Zero::zero() {
                if q.bit(0) { r = (r * &a) % &n; }
                q >>= 1;
                a = (&a * &a) % &n;
            }
            r
        }

        pub fn miller_rabin(n: BigUint) -> Result<bool, Box<dyn Error>> {
            if n == Zero::zero() { return Ok(true); }
            if !n.bit(0) || n == One::one() { return Ok(false); }
            let mut rng = rand::thread_rng();
            let mut pass = false;
            let mut d: BigUint = &n - 1.to_biguint().unwrap();
            while d.bit(0) { d >>= 1; }
            let mut tmp = d.clone();
            for i in 0..CONFIG.read().unwrap().rounds {
                d = tmp.clone();
                pass = false;
                let mut m = fast_Modular_exponent(rng.gen_biguint_range(&Zero::zero(), &(&n - 2.to_biguint().unwrap())) + 2.to_biguint().unwrap(), d.clone(), n.clone());
                if m == One::one() { continue; } else {
                    while d < n {
                        if m == &n - 1.to_biguint().unwrap() {
                            pass = true;
                            break;
                        }
                        m = (&m * &m) % &n;
                        d <<= 1;
                    }
                    if !pass { return Ok(false); }
                }
            }
            Ok(true)
        }

        pub fn generate() -> BigUint {
            let mut rng = rand::thread_rng();
            let low = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_min);
            let high = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_max);
            rng.gen_biguint_range(&low, &high)
        }
    }
}