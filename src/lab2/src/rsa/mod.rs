pub mod config;

pub mod rsa {
    pub mod prime_gen {
        use num_bigint::{BigUint, RandBigInt, ToBigUint};
        use num_traits::*;
        use crate::rsa::config::config::*;

        // use crate::rsa::rsa::config::{CONFIG, Config};
        pub fn generate() -> BigUint {
            let mut rng = rand::thread_rng();
            let low = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_min);
            let high = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_max);
            rng.gen_biguint_range(&low, &high)
        }
    }
}