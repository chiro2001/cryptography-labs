use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use num::Integer;
use clap::Parser;
use num_bigint::{BigInt, Sign, ToBigInt, ToBigUint};
use num_traits::{One, Pow, Zero};
use prime_gen::PrimeError;
use crate::rsa::config::CONFIG_DEF;
use crate::rsa::keys::{Key, KeySet};
use crate::rsa::keys::key_data::KeyData;
use crate::rsa::keys::key_pair::KeyPair;

pub mod config;
pub mod prime_gen;
pub mod keys;

#[derive(Debug)]
pub enum RunMode {
    Generate,
    Encode,
    Decode,
    Test,
}

#[derive(Debug, Parser)]
pub struct RSA {
    #[clap(short, long, value_parser, default_value = CONFIG_DEF.mode.as_str(), help = "Run mode", value_parser = ["generate", "encode", "decode", "test"])]
    pub mode: String,
    #[clap(short, long, value_parser, default_value = CONFIG_DEF.key.as_str(), help = "Key path, generate `path' and `path.pub'")]
    pub key: String,
    #[clap(short, long, value_parser, default_value = CONFIG_DEF.comment.as_str(), help = "Attach comment to key files")]
    pub comment: String,
    #[clap(long, value_parser, default_value_t = CONFIG_DEF.base64, help = "Output key in base64 format")]
    pub base64: bool,
    #[clap(short, long, value_parser, default_value = CONFIG_DEF.input.as_str(), help = "Input filename")]
    pub input: String,
    #[clap(short, long, value_parser, default_value = CONFIG_DEF.output.as_str(), help = "Output filename")]
    pub output: String,
    #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_min, help = "Min prime bits")]
    pub prime_min: u32,
    #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_max, help = "Max prime bits")]
    pub prime_max: u32,
    #[clap(short, long, value_parser, default_value_t = CONFIG_DEF.rounds, help = "Miller Rabin calculate rounds")]
    pub rounds: u32,
    #[clap(short, long, value_parser, default_value_t = CONFIG_DEF.time_max, help = "Max time in mill seconds that trying to generate a prime")]
    pub time_max: i64,
    #[clap(short, long, value_parser, default_value_t = CONFIG_DEF.silent, help = "Disable log output")]
    pub silent: bool,
    #[clap(long, value_parser, default_value_t = CONFIG_DEF.retry, help = "Retry when failed to generate primes")]
    pub retry: bool,
    #[clap(long, value_parser, default_value_t = CONFIG_DEF.threads, help = "Generate primes using <THREADS> threads")]
    pub threads: usize,
}

impl RSA {
    pub fn get(&self) -> &RSA {
        self
    }

    pub fn copy(&self) -> RSA {
        RSA {
            prime_min: self.prime_min,
            prime_max: self.prime_max,
            input: self.input.clone(),
            output: self.output.clone(),
            base64: self.base64,
            rounds: self.rounds,
            time_max: self.time_max,
            mode: self.mode.clone(),
            silent: self.silent,
            key: self.key.clone(),
            threads: self.threads,
            retry: self.retry,
            comment: self.comment.clone(),
        }
    }

    pub fn set(&mut self, other: RSA) {
        *self = other;
    }

    fn reader(&self) -> Box<dyn Read> {
        match self.input.as_str() {
            "stdin" => Box::new(io::stdin()),
            f => Box::new(File::open(f).unwrap())
        }
    }

    fn writer(&mut self) -> Box<dyn Write> {
        match self.output.as_str() {
            "stdout" => {
                self.silent = true;
                Box::new(io::stdout())
            }
            f => Box::new(File::create(f).unwrap())
        }
    }

    fn run_mode(&self) -> RunMode {
        match self.mode.as_str() {
            "encode" => Ok(RunMode::Encode),
            "decode" => Ok(RunMode::Decode),
            "generate" => Ok(RunMode::Generate),
            "test" => Ok(RunMode::Test),
            _ => Err(())
        }.unwrap()
    }

    pub fn euler(p: &BigInt, q: &BigInt) -> BigInt { (p - 1.to_bigint().unwrap()) * (q - 1.to_bigint().unwrap()) }

    fn extended_euclid(a: &BigInt, b: &BigInt, x: &BigInt, y: &BigInt) -> (BigInt, BigInt, BigInt) {
        if b.is_zero() {
            return (a.clone(), 1.to_bigint().unwrap(), 0.to_bigint().unwrap());
        }
        let (d, x2, y2) = RSA::extended_euclid(b, &(a % b), y, x);
        return (d, y2.clone(), x2 - a / b * &y2);
    }

    pub fn mod_reverse(a: &BigInt, b: &BigInt) -> BigInt {
        let d = RSA::extended_euclid(a, b, &Zero::zero(), &One::one());
        if d.0.is_one() {
            (d.1 % b + b) % b
        } else {
            Zero::zero()
        }
    }

    pub fn generate_key(&self) -> Result<KeySet, PrimeError> {
        let low = 2.to_biguint().unwrap().pow(self.prime_min);
        let high = 2.to_biguint().unwrap().pow(self.prime_max);
        let (p, q) = (self.generate_prime(&low, &high)?, self.generate_prime(&low, &high)?);
        let n = &p * &q;
        let f = RSA::euler(&p, &q);
        let mut e;
        loop {
            e = self.generate_prime(&1.to_biguint().unwrap(), &f.to_biguint().unwrap())?;
            if f.gcd(&e).is_one() { break; }
        }
        let d = RSA::mod_reverse(&e, &f);
        self.check_key_set(&d, &e, &f);
        Ok(KeySet { public: Key { m: n.clone(), base: e }, private: Key { m: n.clone(), base: d } })
    }

    pub fn check_key_set(&self, d: &BigInt, e: &BigInt, f: &BigInt) {
        let res = (d * e) % f;
        if !self.silent {
            println!("(d * e) % f = {} % {} = {}", d * e, f, res);
        }
        assert!(res.is_one());
    }

    fn read_source(reader: &mut dyn Read, bytes: usize) -> Vec<u8> {
        let mut source = [0 as u8; 1];
        let mut res = Vec::new();
        loop {
            match reader.read(source.as_mut()) {
                Ok(n) => match n {
                    0 => break,
                    _ => {
                        res.push(source[0]);
                        if res.len() >= bytes { break; }
                    }
                },
                _ => break
            }
        }
        res
    }

    fn get_group_size_byte(n: &BigInt) -> usize { f64::pow(2 as f64, ((n.bits() as usize / 8) as f64).log2().ceil()) as usize / 2 }

    pub fn process(reader: &mut dyn Read, writer: &mut dyn Write, mode: RunMode, key: Key) {
        let group_size = RSA::get_group_size_byte(&key.m) * match mode {
            RunMode::Decode => 2,
            _ => 1
        };
        loop {
            let source = RSA::read_source(reader, group_size);
            if source.is_empty() { break; }
            let data = BigInt::from_bytes_le(Sign::Plus, source.as_slice());
            let res = RSA::fast_modular_exponent(data.clone(), key.base.clone(), key.m.clone());
            let mut res_data = res.to_bytes_le().1.clone();
            let res_data_len = res_data.len();
            match mode {
                RunMode::Encode => for _ in 0..(group_size * 2 - res_data_len) { res_data.push(0); }
                _ => {}
            };
            writer.write_all(res_data.as_slice()).unwrap();
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match self.run_mode() {
            RunMode::Generate => {
                let key_set = self.generate_key()?;
                if !self.silent { println!("get keys: {:?}", key_set); }
                let mut key_pair = KeyPair {
                    public: KeyData::new_public(key_set.public, "Hello RSA!".to_string()),
                    private: KeyData::new_private(key_set.private, "Hello RSA!".to_string()),
                };
                if !self.silent { println!("get key_pair: {:?}", key_pair); }
                key_pair.save(self.key.clone(), self.base64).unwrap();
                if !self.silent { println!("Generated key files: {}, {}", self.key.clone(), self.key.clone() + ".pub"); }
            }
            RunMode::Test => {
                let key_pair = KeyPair::from(self.key.clone());
                if !self.silent { println!("get key_pair: {:?}", key_pair); }
                let m = BigInt::from(88);
                let c = RSA::fast_modular_exponent(m.clone(), key_pair.public.key.base.clone(), key_pair.public.key.m.clone());
                let m2 = RSA::fast_modular_exponent(c.clone(), key_pair.private.key.base.clone(), key_pair.private.key.m.clone());
                assert_eq!(m, m2);
                if !self.silent { println!("Test pass"); };
            }
            RunMode::Encode | RunMode::Decode => {
                let mut reader = self.reader();
                let mut writer = self.writer();
                let path = match self.run_mode() {
                    RunMode::Decode => self.key.clone(),
                    _ => self.key.clone() + ".pub"
                };
                let key = KeyData::from(path);
                RSA::process(&mut reader, &mut writer, self.run_mode(), key.key);
                if !self.silent { println!("Done"); };
            }
        }
        Ok(())
    }
}
