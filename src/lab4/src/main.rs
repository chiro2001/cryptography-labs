use std::error::Error;
use clap::Parser;
use num_bigint::{BigInt, BigUint, ToBigUint};
pub use elgamal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let elgamal = ElGamal::parse();
    // let prime = RSA::generate_one_prime(&0.to_biguint().unwrap(), &1000.to_biguint().unwrap(), 20, 3000)?;
    // println!("prime = {}", prime);
    Ok(())
}