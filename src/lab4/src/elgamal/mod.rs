use std::ptr::hash;
use num::Integer;
use num_bigint::{BigInt, RandBigInt, ToBigInt, ToBigUint};
use num_traits::{Num, One, Pow, ToPrimitive};
use rsa::RSA;
use crate::elgamal::keys::ElGamalKey;
use sha256::Sha256Digest;

pub mod config;
pub mod keys;
pub mod sign;

pub use config::*;
pub use keys::*;
pub use sign::*;

pub type ElGamal = RSA;

pub trait ElGamalTrait {
    fn elgamal_generate_key(&self) -> ElGamalKey;
    fn elgamal_sign(data: &Vec<u8>, key: &ElGamalKey) -> ElGamalSign;
    fn hash(src: &Vec<u8>) -> BigInt;
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
        let x = rng.gen_biguint_range(&2.to_biguint().unwrap(), &p_1).to_bigint().unwrap();
        // let x = 127.to_bigint().unwrap();
        let y = g.modpow(&x, &p);
        ElGamalKey { public: ElGamalPublicKey { p: p.clone(), g, y }, private: ElGamalPrivateKey { x } }
    }

    fn hash(src: &Vec<u8>) -> BigInt {
        let decode_radix = 10;
        let hashed_string =
            if decode_radix == 16 {
                src.digest()
            } else {
                BigInt::from_str_radix(String::from_utf8(src.clone()).unwrap().as_str(), 10)
                    .unwrap().to_bytes_le().1.digest()
            };
        let hashed = BigInt::from_str_radix(&hashed_string, 16).unwrap();
        println!("hash({}) = {}", String::from_utf8(src.clone()).unwrap(), hashed);
        hashed
    }

    fn elgamal_sign(data: &Vec<u8>, key: &ElGamalKey) -> ElGamalSign {
        let mut rng = rand::thread_rng();
        let p = &key.public.p;
        let g = &key.public.g;
        let p_1 = p.clone() - 1.to_bigint().unwrap();
        let mut k;
        loop {
            k = rng.gen_biguint_range(&1.to_biguint().unwrap(), &p.to_biguint().unwrap()).to_bigint().unwrap();
            if k.gcd(&p_1).is_one() { break; }
        }
        let r = g.modpow(&k, p);
        let k_inv = RSA::mod_reverse(&k, &p_1);
        let hashed = Self::hash(data);
        let s = (&k_inv * (&hashed - (&key.private.x * &r))) % &p_1;
        println!("k = {}, r = {}, p = {}, p_1 = {}, k_inv = {}, hashed = {}, s = {}, g = {}",
                 k, r, p, p_1, k_inv, hashed, s, g);
        ElGamalSign { r, s }
    }
}