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
        "stdout" => Box::new(io::stdout()),
        f => Box::new(File::create(f).unwrap())
    };
    let mode = match args.mode.as_str() {
        "encode" => Ok(RunMode::Encode),
        "decode" => Ok(RunMode::Decode),
        "generate" => Ok(RunMode::Generate),
        _ => Err(())
    }.unwrap();

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
    println!("Done");
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use num::Integer;
    use num_bigint::{ToBigInt, ToBigUint};
    use num_traits::One;
    use crate::{CONFIG, rsa};
    use crate::prime_gen::generate;
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
}