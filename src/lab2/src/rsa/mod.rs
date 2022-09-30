use num::Integer;
use num_bigint::{BigInt, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use crate::CONFIG;
use crate::prime_gen::{generate, PrimeError};

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

fn euler(p: &BigInt, q: &BigInt) -> BigInt { (p - 1.to_bigint().unwrap()) * (q - 1.to_bigint().unwrap()) }

// fn extended_euclid(a: &BigInt, b: &BigInt) -> BigInt {
//     // if b >= a { return extended_euclid(b, a); }
//     let (mut x1, mut x2, mut x3): (BigInt, BigInt, BigInt) = (1.to_bigint().unwrap(), 0.to_bigint().unwrap(), a.clone());
//     let (mut y1, mut y2, mut y3): (BigInt, BigInt, BigInt) = (0.to_bigint().unwrap(), 1.to_bigint().unwrap(), b.clone());
//     let mut lo = 0;
//     loop {
//         lo += 1;
//         println!("loop {}", lo);
//         if y3.is_zero() { return x3; }
//         let q = &x3 / &y3;
//         println!("x=({}, {}, {}), y=({}, {}, {}), q={}", x1, x2, x3, y1, y2, y3, q);
//         if y3.is_one() { return y2; }
//         let (t1, t2, t3) = (&x1 - &q * &y1, &x2 - &q * &y2, &x3 - q * &y3);
//         x1 = y1.clone();
//         x2 = y2.clone();
//         x3 = y3.clone();
//         y1 = t1;
//         y2 = t2;
//         y3 = t3;
//     }
// }

// fn extended_euclid(a: &BigInt, b: &BigInt) -> BigInt {
//
// }

// fn extended_euclid(a: &BigInt, b: &BigInt, x: &BigInt, y: &BigInt) -> (BigInt, BigInt, BigInt) {
//     println!("extended_euclid({}, {}, {}, {})", a, b, x, y);
//     if b.is_zero() {
//         return (a.clone(), 1.to_bigint().unwrap(), 0.to_bigint().unwrap());
//     }
//     let (d, x2, y2) = extended_euclid(b, &(a % b), y, x);
//     println!("extended_euclid({}, {}, {}, {}): d={}, x={}, y={}", b, &(a % b), y, x, d, x2, y2);
//     return (d, x2.clone(), y2 - a / b * &x2);
// }

fn extended_euclid(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() { return (1.to_bigint().unwrap(), 0.to_bigint().unwrap(), a.clone()); }
    let (mut s0, mut s) = (1.to_bigint().unwrap(), 0.to_bigint().unwrap());
    let (mut t0, mut t) = (0.to_bigint().unwrap(), 1.to_bigint().unwrap());
    let (mut r0, mut r) = (a.clone(), b.clone());
    while !r.is_zero() {
        let q = &r0 / &r;
        let (t1, t2, t3) = (r0.clone(), s0.clone(), t0.clone());
        r0 = r.clone(); r = t1 - &q * r;
        s0 = s.clone(); s = t2 - &q * s;
        t0 = t.clone(); t = t3 - &q * t;
    }
    (s0, t0, r0)
}

pub fn generate_key() -> Result<KeySet, PrimeError> {
    let low = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_min);
    let high = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_max);
    let (p, q) = (generate(&low, &high)?, generate(&low, &high)?);
    // let n = &p * &q;
    let f = euler(&p, &q);
    let mut e;
    loop {
        e = generate(&1.to_biguint().unwrap(), &f.to_biguint().unwrap())?;
        if f.gcd(&e).is_one() { break; }
    }
    // let d = extended_euclid(&e, &f, &Zero::zero(), &One::one());
    let d = extended_euclid(&e, &f);
    // println!("f - (e*d) = {}", &f - (&e * &d));
    println!("e*x = {}, b*y = {}", &e * &d.1, &f * &d.2);
    println!("d - (e*x + b*y) = {}", &d.0 - (&e * &d.1 + &f * &d.2));
    // println!("e*d == 1 mod eu: {} * {} == 1 mod {}", e, d, f);
    Ok(KeySet { public: Key { m: f.clone(), base: e }, private: Key { m: f.clone(), base: d.0 } })
}
