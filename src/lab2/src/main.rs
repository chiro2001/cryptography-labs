mod rsa;

use std::error::Error;
use clap::Parser;
use crate::rsa::config::config::*;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
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
    use crate::rsa::config::config::*;
    use crate::rsa::rsa::*;

    fn init() -> Result<(), Box<dyn Error>> {
        if !CONFIG.is_set().unwrap() {
            CONFIG.set(CONFIG_DEF.copy())?;
        }
        println!("Use default config: {:?}", CONFIG.read()?.get());
        Ok(())
    }

    #[test]
    fn gen_prime() -> Result<(), Box<dyn Error>> {
        init()?;
        let prime = prime_gen::generate();
        println!("got prime: {:?}", prime);
        Ok(())
    }
}