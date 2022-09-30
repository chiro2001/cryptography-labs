use std::io::{Read, Write};
use num::Integer;
use num_bigint::{BigInt, Sign, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use crate::CONFIG;
use crate::prime_gen::{fast_modular_exponent, generate, PrimeError};

pub mod config;
pub mod prime_gen;

#[derive(Debug)]
pub struct Key {
    pub base: BigInt,
    pub m: BigInt,
}

#[derive(Debug)]
pub struct KeySet {
    pub public: Key,
    pub private: Key,
}

#[derive(Debug)]
pub enum RunMode {
    Generate,
    Encode,
    Decode,
}

pub fn euler(p: &BigInt, q: &BigInt) -> BigInt { (p - 1.to_bigint().unwrap()) * (q - 1.to_bigint().unwrap()) }

fn extended_euclid(a: &BigInt, b: &BigInt, x: &BigInt, y: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        return (a.clone(), 1.to_bigint().unwrap(), 0.to_bigint().unwrap());
    }
    let (d, x2, y2) = extended_euclid(b, &(a % b), y, x);
    return (d, y2.clone(), x2 - a / b * &y2);
}

pub fn mod_reverse(a: &BigInt, b: &BigInt) -> BigInt {
    let d = extended_euclid(a, b, &Zero::zero(), &One::one());
    if d.0.is_one() {
        (d.1 % b + b) % b
    } else {
        Zero::zero()
    }
}

pub fn generate_key() -> Result<KeySet, PrimeError> {
    let low = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_min);
    let high = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_max);
    let (p, q) = (generate(&low, &high)?, generate(&low, &high)?);
    let f = euler(&p, &q);
    let mut e;
    loop {
        e = generate(&1.to_biguint().unwrap(), &f.to_biguint().unwrap())?;
        if f.gcd(&e).is_one() { break; }
    }
    let d = mod_reverse(&e, &f);
    Ok(KeySet { public: Key { m: f.clone(), base: e }, private: Key { m: f.clone(), base: d } })
}

pub fn check_key_set(keys: &KeySet) {
    assert_eq!(keys.private.m, keys.public.m);
    let f = keys.public.m.clone();
    let e = keys.public.base.clone();
    let d = keys.private.base.clone();
    let res = (&d * &e) % &f;
    println!("(d * e) % f = {} % {} = {}", &d * &e, f, res);
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

fn get_group_size(n: &BigInt) -> usize { n.bits() as usize }

pub fn process(reader: &mut dyn Read, writer: &mut dyn Write, mode: RunMode, key: Key) {
    let group_size = get_group_size(&key.m);
    loop {
        let source = read_source(reader, group_size);
        if source.is_empty() { break; }
        // println!("source: {:?}", source);
        let data = BigInt::from_bytes_le(Sign::Plus, source.as_slice());
        let res = fast_modular_exponent(data.clone(), key.m.clone(), key.base.clone());
        // println!("{:?} -> {:?}", data, res);
        writer.write_all(res.to_bytes_le().1.as_slice()).unwrap();
    }
}