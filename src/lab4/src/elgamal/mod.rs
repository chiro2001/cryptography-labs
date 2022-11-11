use std::error::{Error};
use std::{io};
use std::fmt::{Display, Formatter};
use num::Integer;
use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt, ToBigUint};
use num_traits::{One, Pow, Signed};
use rsa::RSA;
use crate::elgamal::keys::ElGamalKey;
use sha2::Sha256;
use sha2::Digest;
use clap::Parser;

pub mod config;
pub mod keys;
pub mod sign;
pub mod key_reader;

pub use config::*;
pub use keys::*;
pub use sign::*;
pub use key_reader::*;

use rsa::*;

rsa_t!(CONFIG_DEF, ElGamalFake);

impl From<ElGamalFake> for ElGamal {
    fn from(e: ElGamalFake) -> Self {
        Self {
            mode: e.mode,
            key: e.key,
            comment: e.comment,
            binary: e.binary,
            input: e.input,
            output: e.output,
            prime_min: e.prime_min,
            prime_max: e.prime_max,
            rounds: e.rounds,
            time_max: e.time_max,
            silent: e.silent,
            retry: e.retry,
            threads: e.threads,
        }
    }
}

pub type ElGamal = RSA;

#[derive(Debug, Clone)]
pub enum RunMode {
    Generate,
    Sign,
    Check,
}

#[derive(Debug)]
pub enum ElgamalError {
    CheckError
}

impl Display for ElgamalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElgamalError::CheckError => f.write_str("Check sign failed!"),
            // _ => f.write_str("Unknown Error!")
        }
    }
}

impl Error for ElgamalError {}

pub trait ElGamalTrait {
    fn elgamal_generate_key(&self) -> ElGamalKey;
    fn elgamal_sign(&self, key: &ElGamalKey) -> ElGamalSign;
    fn data_hashed(&self) -> BigInt;
    fn elgamal_check(&self, sign: &ElGamalSign, key: &ElGamalPublicKey) -> bool;
    fn elgamal_run_mode(&self) -> RunMode;
    fn run_elgamal(&mut self) -> Result<(), Box<dyn Error>>;
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
        // ElGamalKey {
        //     public: ElGamalPublicKey {
        //         p: 467.to_bigint().unwrap(),
        //         g: 2.to_bigint().unwrap(),
        //         y: 132.to_bigint().unwrap(),
        //     },
        //     private: ElGamalPrivateKey {
        //         x: 127.to_bigint().unwrap()
        //     },
        // }
    }

    fn elgamal_sign(&self, key: &ElGamalKey) -> ElGamalSign {
        let mut rng = rand::thread_rng();
        let p = &key.public.p;
        let g = &key.public.g;
        let p_1 = p.clone() - 1.to_bigint().unwrap();
        let mut k;
        loop {
            k = rng.gen_biguint_range(&1.to_biguint().unwrap(), &p.to_biguint().unwrap()).to_bigint().unwrap();
            if k.gcd(&p_1).is_one() { break; }
        }
        // k = 213.to_bigint().unwrap();
        let r = g.modpow(&k, p);
        let k_inv = RSA::mod_reverse(&k, &p_1);
        let hashed = self.data_hashed();
        let mut s = (&k_inv * (&hashed - (&key.private.x * &r))) % &p_1;
        while s.is_negative() {
            s = s + &p_1;
        }
        if !SILENT.read().unwrap().clone() {
            println!("k = {}, r = {}, p = {}, p_1 = {}, k_inv = {}, hashed = {}, s = {}, g = {}",
                     k, r, p, p_1, k_inv, hashed, s, g);
        }
        ElGamalSign { r, s }
    }

    fn data_hashed(&self) -> BigInt {
        let mut hasher = Sha256::new();
        let mut reader = self.reader();
        let _n = io::copy(&mut reader, &mut hasher).unwrap();
        let hash = hasher.finalize();
        let hashed = BigInt::from_bytes_le(Sign::Plus, &hash);
        hashed
    }

    fn elgamal_check(&self, sign: &ElGamalSign, key: &ElGamalPublicKey) -> bool {
        let hashed = self.data_hashed();
        let left = (&key.y.modpow(&sign.r, &key.p) * &sign.r.modpow(&sign.s, &key.p)) % &key.p;
        let right = key.g.modpow(&hashed, &key.p);
        if !SILENT.read().unwrap().clone() {
            println!("{}^{} * {}^{} mod {} =?= {}^{} mod {}", key.y, sign.r, sign.r, sign.s, key.p, key.g, hashed, key.p);
            println!("left =?= right  |  {} =?= {}", &left, &right);
        }
        left == right
    }

    fn elgamal_run_mode(&self) -> RunMode {
        match self.mode.as_str() {
            "generate" => Ok(RunMode::Generate),
            "sign" => Ok(RunMode::Sign),
            "check" => Ok(RunMode::Check),
            _ => Err("Unknown run mode! available: generate(default), sign, check")
        }.unwrap()
    }

    fn run_elgamal(&mut self) -> Result<(), Box<dyn Error>> {
        if self.output == "stdout" {
            self.silent = true
        }
        match self.elgamal_run_mode() {
            RunMode::Generate => {
                let mut key = self.elgamal_generate_key();
                if !self.silent { println!("generated key: {:#?}", key); }
                key.save(self.key.clone(), !self.binary).unwrap();
            }
            RunMode::Sign => {
                let key = ElGamalKey::from(self.key.clone());
                assert!(!key.is_empty(), "Private & Public key must be provided!");
                let mut sign = self.elgamal_sign(&key);
                sign.save(self.key.clone() + ".sig", !self.binary).unwrap();
            }
            RunMode::Check => {
                let key = ElGamalKey::from(self.key.clone());
                let key_pub = key.public;
                assert!(!key_pub.is_empty(), "Public key must be provided!");
                let sign = ElGamalSign::from(self.key.clone() + ".pub");
                let result = self.elgamal_check(&sign, &key_pub);
                if !self.silent {
                    if !result {
                        println!("Check failed!")
                    } else {
                        println!("Check passed!")
                    }
                }
                if !result {
                    return Err(Box::new(ElgamalError::CheckError));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{CONFIG_DEF, ElGamal, ElGamalTrait, Savable};
    use crate::keys::ElGamalKey;

    #[test]
    fn test_key_save_load() {
        let r: &ElGamal = CONFIG_DEF.get();
        let mut key_save = r.elgamal_generate_key();
        key_save.save(r.key.clone(), true).unwrap();
        let key_load = ElGamalKey::from(r.key.clone());
        println!("save: {:#?}", key_save);
        println!("load: {:#?}", key_load);
        assert_eq!(key_load, key_save);
    }
}