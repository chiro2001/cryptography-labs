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
    use std::io::Read;
    use elgamal::ElGamal;
    use elgamal::*;
    use sha256::Sha256Digest;

    #[test]
    fn test_generate_key() -> Result<(), Box<dyn Error>> {
        let r: &ElGamal = CONFIG_DEF.get();
        let key = r.elgamal_generate_key();
        println!("generated key: {:#?}", key);
        Ok(())
    }

    #[test]
    fn test_hash_data() -> Result<(), Box<dyn Error>> {
        let r: &ElGamal = CONFIG_DEF.get();
        let mut reader = r.reader();
        let mut data: [u8; 9] = [0; 9];
        reader.read(&mut data).unwrap();
        let val = data.digest();
        println!("hash({}) = {}", String::from_utf8(data.to_vec()).unwrap(), val);
        Ok(())
    }
}