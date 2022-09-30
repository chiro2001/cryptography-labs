mod rsa;

use std::borrow::Borrow;
use std::error::Error;
use std::ops::DerefMut;
use clap::Parser;
use crate::rsa::rsa::config::CONFIG;

fn main() -> Result<(), Box<dyn Error>> {
    let config = rsa::rsa::config::Config::parse();
    println!("Run with config: {:#?}", config);
    CONFIG.write()?.set(config);
    println!("Run with CONFIG: {:#?}", CONFIG.read().as_ref().unwrap().as_ref());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::rsa::*;
    #[test]
    fn gen_prime() {
        let prime = rsa::prime_gen::generate();
        println!("got prime: {:?}", prime);
    }
}