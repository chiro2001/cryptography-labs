mod rsa;

use std::error::Error;
use clap::Parser;
use crate::rsa::rsa::config::CONFIG;

fn main() -> Result<(), Box<dyn Error>> {
    let config = rsa::rsa::config::Config::parse();
    if !CONFIG.is_set().unwrap() {
        CONFIG.set(config)?;
    } else {
        CONFIG.write()?.set(config);
    }
    println!("Run with CONFIG: {:?}", CONFIG.read().as_ref().unwrap().as_ref());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::CONFIG;
    use crate::rsa::*;
    use crate::rsa::rsa::config::CONFIG_DEF;

    fn init() -> Result<(), Box<dyn Error>> {
        if !CONFIG.is_set().unwrap() {
            CONFIG.set(*CONFIG_DEF)?;
        }
        Ok(())
    }

    #[test]
    fn gen_prime() -> Result<(), Box<dyn Error>> {
        init()?;
        let prime = rsa::prime_gen::generate();
        println!("got prime: {:?}", prime);
        Ok(())
    }
}