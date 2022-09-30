mod rsa;

use std::error::Error;
use clap::Parser;
use num_bigint::ToBigUint;
use crate::rsa::config::config::*;
use crate::rsa::generate_key;
use crate::rsa::prime_gen::prime_gen;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    if !CONFIG.is_set().unwrap() {
        CONFIG.set(config)?;
    } else {
        CONFIG.write()?.set(config);
    }
    println!("Run with CONFIG: {:?}", CONFIG.read().as_ref().unwrap().as_ref());
    // let low = 2.to_bigint().unwrap().pow(CONFIG.read().unwrap().prime_min);
    // let high = 2.to_bigint().unwrap().pow(CONFIG.read().unwrap().prime_max);
    // let prime = prime_gen::generate(&low, &high).unwrap();
    // println!("got prime: {:?}", prime);
    let keys = generate_key()?;
    println!("get keys: {:?}", keys);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use num_bigint::ToBigUint;
    use crate::CONFIG;
    use crate::rsa::config::config;
    use crate::rsa::prime_gen::prime_gen;
    use crate::rsa::prime_gen::prime_gen::miller_rabin;

    #[test]
    fn gen_prime() -> Result<(), Box<dyn Error>> {
        config::use_default()?;
        let low = 2.to_bigint().unwrap().pow(CONFIG.read().unwrap().prime_min);
        let high = 2.to_bigint().unwrap().pow(CONFIG.read().unwrap().prime_max);
        let prime = prime_gen::generate(&low, &high).unwrap();
        println!("got prime: {:?}", prime);
        Ok(())
    }

    #[test]
    fn test_miller_rabin() -> Result<(), Box<dyn Error>> {
        config::use_default()?;
        let res = (0xfffffff0 as u32..0xffffffff as u32)
            .map(|x| (x, miller_rabin(&x.to_bigint().unwrap()).unwrap()))
            .filter(|x| x.1)
            .collect::<Vec<_>>();
        println!("result: {:?}", res);
        Ok(())
    }
}