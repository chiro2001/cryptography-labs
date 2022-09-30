pub mod rsa {
    pub mod config {
        use clap::Parser;
        use lazy_static::lazy_static;
        use mut_static::MutStatic;

        #[derive(Debug, Parser, Copy, Clone)]
        pub struct Config {
            #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_min, help = "Min prime bits")]
            pub prime_min: usize,
            #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_max, help = "Max prime bits")]
            pub prime_max: usize,
        }

        impl Config {
            pub fn get(&self) -> &Config {
                self
            }
            pub fn set(&mut self, other: Config) {
                *self = other;
            }
        }

        lazy_static! {
            pub static ref CONFIG_DEF: Config = Config { prime_min: 14, prime_max: 1024 };
            pub static ref CONFIG: MutStatic<Config> = {
                MutStatic::new()
            };
        }
    }

    pub mod prime_gen {
        use num_bigint::BigUint;
        use num_traits::*;
        use std::mem::replace;
        use std::ops::DerefMut;

        use crate::rsa::rsa::config::{CONFIG, Config};

        // Calculate large fibonacci numbers.
        fn fib(n: usize) -> BigUint {
            let mut f0: BigUint = Zero::zero();
            let mut f1: BigUint = One::one();
            for _ in 0..n {
                let f2 = f0 + &f1;
                // This is a low cost way of swapping f0 with f1 and f1 with f2.
                f0 = replace(&mut f1, f2);
            }
            f0
        }

        pub fn generate() -> BigUint {
            CONFIG.write().unwrap().prime_min = 1;
            println!("config now: {:?}", CONFIG.read().unwrap().as_ref());
            fib(3000)
        }
    }
}