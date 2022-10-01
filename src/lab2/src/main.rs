extern crate core;

mod rsa;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::str::FromStr;
use clap::Parser;
use num_bigint::BigInt;
use crate::rsa::config::config::*;
use crate::rsa::{generate_key, Key, process, RunMode};
use crate::rsa::prime_gen::prime_gen;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Config::parse();
    if !CONFIG.is_set().unwrap() {
        CONFIG.set(args)?;
    } else {
        CONFIG.write()?.set(args);
    }
    println!("Run with CONFIG: {:?}", CONFIG.read().as_ref().unwrap().as_ref());

    let args: Config = CONFIG.read().unwrap().copy();

    let mut reader: Box<dyn Read> = match args.input.as_str() {
        "stdin" => Box::new(io::stdin()),
        f => Box::new(File::open(f).unwrap())
    };
    let mut writer: Box<dyn Write> = match args.output.as_str() {
        "stdout" => {
            CONFIG.write().unwrap().silent = true;
            Box::new(io::stdout())
        }
        f => Box::new(File::create(f).unwrap())
    };
    let mode = match args.mode.as_str() {
        "encode" => Ok(RunMode::Encode),
        "decode" => Ok(RunMode::Decode),
        "generate" => Ok(RunMode::Generate),
        _ => Err(())
    }.unwrap();

    let keys = generate_key()?;
    if !CONFIG.read().unwrap().silent {
        println!("get keys: {:?}", keys);
    }

    match mode {
        RunMode::Generate => {
            let keys = generate_key()?;
            println!("get keys: {:?}", keys);
        }
        RunMode::Encode | RunMode::Decode => {
            process(&mut reader, &mut writer, mode, Key {
                base: BigInt::from_str("1443457866423536847339250332650263408873996464973571486540133220728631678129").unwrap(),
                m: BigInt::from_str("2053363943376975333926026436653596044954830140664527385358194472132153005680").unwrap(),
            })
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;
    use std::io;
    use num::Integer;
    use num_bigint::{BigInt, Sign, ToBigInt, ToBigUint};
    use num_traits::One;
    use crate::{CONFIG, generate_key, Key, process, rsa, RunMode};
    use crate::prime_gen::{fast_modular_exponent, generate};
    use crate::rsa::{check_key_set, KeySet, mod_reverse};
    use crate::rsa::config::config;
    use crate::rsa::prime_gen::prime_gen;
    use crate::rsa::prime_gen::prime_gen::miller_rabin;

    #[test]
    fn gen_prime() -> Result<(), Box<dyn Error>> {
        config::use_default()?;
        let low = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_min);
        let high = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_max);
        let prime = prime_gen::generate(&low, &high).unwrap();
        println!("got prime: {:?}", prime);
        Ok(())
    }

    #[test]
    fn test_miller_rabin() -> Result<(), Box<dyn Error>> {
        config::use_default()?;
        let res = (0xffffff00 as u32..0xffffffff as u32)
            .map(|x| (x, miller_rabin(&x.to_bigint().unwrap()).unwrap()))
            .filter(|x| x.1)
            .map(|x| x.0)
            .collect::<Vec<_>>();
        println!("result: {:?}", res);
        Ok(())
    }

    #[test]
    fn test_mod_reverse() -> Result<(), Box<dyn Error>> {
        config::use_default()?;
        let low = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_min);
        let high = 2.to_biguint().unwrap().pow(CONFIG.read().unwrap().prime_max);
        let (p, q) = (generate(&low, &high)?, generate(&low, &high)?);
        let f = rsa::euler(&p, &q);
        let mut e;
        loop {
            e = generate(&1.to_biguint().unwrap(), &f.to_biguint().unwrap())?;
            if f.gcd(&e).is_one() { break; }
        }
        let d = rsa::mod_reverse(&e, &f);
        let res = (&d * &e) % &f;
        println!("(d * e) % f = {} % {} = {}", &d * &e, f, res);
        assert!(res.is_one());
        Ok(())
    }

    #[test]
    fn test_from_bytes() {
        let data = "114514".as_bytes();
        let d = BigInt::from_bytes_le(Sign::Plus, data);
        println!("{:?} => {:?}", data, d);
    }

    #[test]
    fn function_test() -> Result<(), Box<dyn Error>> {
        config::use_default()?;
        let keys = generate_key()?;
        println!("get keys: {:?}", keys);
        let (key_public, key_private) = (keys.public, keys.private);
        let mut reader = File::open(&CONFIG.read().unwrap().input).unwrap();
        let mut writer_temp = File::create(&CONFIG.read().unwrap().output).unwrap();
        process(&mut reader, &mut writer_temp, RunMode::Encode, key_public);
        let mut reader_temp = File::open(&CONFIG.read().unwrap().output).unwrap();
        let mut writer = io::stdout();
        process(&mut reader_temp, &mut writer, RunMode::Decode, key_private);
        println!("\nDone.");
        Ok(())
    }

    #[test]
    fn test_simple_data() -> Result<(), Box<dyn Error>> {
        let (p, q) = (17.to_bigint().unwrap(), 11.to_bigint().unwrap());
        let f = (&q - 1.to_bigint().unwrap()) * (&p - 1.to_bigint().unwrap());
        let e = 7.to_bigint().unwrap();
        let d = mod_reverse(&e, &f);
        let n = &p * &q;
        check_key_set(&d, &e, &f);
        let keys = KeySet { public: Key { m: n.clone(), base: e }, private: Key { m: n.clone(), base: d } };
        println!("keys: {:?}", keys);
        let m = BigInt::from(88);
        let c = fast_modular_exponent(m.clone(), keys.public.base, keys.public.m);
        let m2 = fast_modular_exponent(c.clone(), keys.private.base, keys.private.m);
        println!("m={}, c={}, m2={}", m, c, m2);
        Ok(())
    }

    #[test]
    fn test_vec_push() {
        let mut v = vec![1, 2, 3, 4];
        v.push(0);
        v.append(&mut vec![5]);
        println!("v={:?}", v);
    }

    #[test]
    fn test_num_bits() {
        let n = BigInt::from(0x11234567855aai64);
        let l = n.to_bytes_le().1;
        let b = n.to_bytes_be().1;
        println!("n: {}, b: {:x?}, l: {:x?}", n, b, l);
    }
}