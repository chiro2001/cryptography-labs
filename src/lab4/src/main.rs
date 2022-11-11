use std::error::Error;
use clap::Parser;
pub use elgamal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let elgamal = ElGamal::parse();
    // let prime = RSA::generate_one_prime(&0.to_biguint().unwrap(), &1000.to_biguint().unwrap(), 20, 3000)?;
    // println!("prime = {}", prime);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use elgamal::ElGamal;
    use elgamal::*;

    #[test]
    fn generate_key() -> Result<(), Box<dyn Error>> {
        let r: &ElGamal = CONFIG_DEF.get();
        let key = r.elgamal_generate_key();
        println!("generated key: {:#?}", key);
        Ok(())
    }
}