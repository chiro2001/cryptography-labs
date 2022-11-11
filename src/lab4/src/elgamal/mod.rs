use num_bigint::{RandBigInt, ToBigInt, ToBigUint};
use num_traits::{One, Pow};
use rsa::RSA;
use crate::elgamal::keys::ElGamalKey;

pub mod config;
pub mod keys;

pub use config::*;
pub use keys::*;

pub type ElGamal = RSA;

pub trait ElGamalTrait {
    fn elgamal_generate_key(&self) -> ElGamalKey;
}

impl ElGamalTrait for ElGamal {
    /**
    1.随机生成一个素数p
    2.令s=2p+1，并判断s是否是一个素数
    3.如果是，那么s就是一个安全素数，否则转1
    4.在安全素数s下，随机选取g，1<g<s-1.
    5.如果g^2 mod s!=1 且 g ^ p mod s!=1，那么g就是本原元
     */
    fn elgamal_generate_key(&self) -> ElGamalKey {
        let mut rng = rand::thread_rng();
        // random select big prime `p` and save prime s
        let mut s;
        let mut p;
        loop {
            let low = 2.to_biguint().unwrap().pow(self.prime_min);
            let high = 2.to_biguint().unwrap().pow(self.prime_max);
            p = self.generate_prime(&low, &high).unwrap();
            s = &p * 2 + 1;
            if RSA::miller_rabin(&s, self.rounds).unwrap() {
                break;
            } else {
                // println!("{} is not prime!", s);
                // p = 233.to_bigint().unwrap();
                // s = 467.to_bigint().unwrap();
                // break;
            }
        }
        // find root for this prime `q`
        let p_1 = (&p - 1.to_bigint().unwrap()).to_biguint().unwrap();
        let mut g;
        loop {
            g = self.generate_prime(&1.to_biguint().unwrap(), &p_1).unwrap();
            if (!g.modpow(&2.to_bigint().unwrap(), &s).is_one()) && (!g.modpow(&p, &s).is_one()) { break; }
        }
        // g = 2.to_bigint().unwrap();
        let x = rng.gen_biguint_range(&1.to_biguint().unwrap(), &p_1).to_bigint().unwrap();
        // let x = 127.to_bigint().unwrap();
        let y = g.modpow(&x, &p);
        ElGamalKey { public: ElGamalPublicKey { p: p.clone(), g, y }, private: ElGamalPrivateKey { x } }
    }
}