use std::io::{Read, Write};
use num::Integer;
use num_bigint::{BigInt, Sign, ToBigInt, ToBigUint};
use num_traits::{One, Pow, Zero};
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
    let n = &p * &q;
    let f = euler(&p, &q);
    let mut e;
    loop {
        e = generate(&1.to_biguint().unwrap(), &f.to_biguint().unwrap())?;
        if f.gcd(&e).is_one() { break; }
    }
    let d = mod_reverse(&e, &f);
    check_key_set(&d, &e, &f);
    Ok(KeySet { public: Key { m: n.clone(), base: e }, private: Key { m: n.clone(), base: d } })
}

pub fn check_key_set(d: &BigInt, e: &BigInt, f: &BigInt) {
    let res = (d * e) % f;
    println!("(d * e) % f = {} % {} = {}", d * e, f, res);
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
    let group_size = get_group_size_byte(&key.m) * match mode {
        RunMode::Decode => 2,
        _ => 1
    };
    // let group_size = match mode {
    //     RunMode::Decode => 32,
    //     _ => 16
    // };
    println!("group_size = {}", group_size);
    loop {
        let source = read_source(reader, group_size);
        if source.is_empty() { break; }
        let data = BigInt::from_bytes_le(Sign::Plus, source.as_slice());
        let res = fast_modular_exponent(data.clone(), key.base.clone(), key.m.clone());
        let mut res_data = res.to_bytes_le().1.clone();
        let res_data_len = res_data.len();
        // for _ in res_data_len..group_size { res_data.push(0); }
        println!("\nsource size: {:?}, res size: {:?} | {:?}", source.len(), res.bits(), res.to_bytes_le().1.len());
        match mode {
            RunMode::Encode => println!("C = ({:?} ^ {:?}) % {:?} = {:?}", data, key.base, key.m, res),
            RunMode::Decode => println!("M = ({:?} ^ {:?}) % {:?} = {:?}", data, key.base, key.m, res),
            _ => {}
        }
        // println!("{:?} -> {:?}", data, res);
        writer.write_all(res.to_bytes_le().1.as_slice()).unwrap();
    }
}