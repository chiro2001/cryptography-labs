use std::error::Error;
use std::io::{Write};
use std::fs::File;
use num_bigint::BigInt;
use num_traits::Zero;
use rsa::keys::KeyWriter;


pub trait Savable {
    fn get_file_writer(path: String, base64_output: bool, key_type: &str) -> Box<dyn Write> {
        match base64_output {
            true => {
                let mut key_writer = KeyWriter::from(Box::new(File::create(path).unwrap()));
                key_writer.header = format!("-----BEGIN ELGAMAL-RS {}-----", key_type);
                key_writer.footer = format!("-----END ELGAMAL-RS {}-----", key_type);
                Box::new(base64::write::EncoderWriter::new(
                    key_writer,
                    base64::STANDARD))
            }
            false => Box::new(File::create(path).unwrap())
        }
    }

    fn save(&mut self, path: String, base64_output: bool) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct ElGamalKey {
    pub public: ElGamalPublicKey,
    pub private: ElGamalPrivateKey,
}

#[derive(Debug)]
pub struct ElGamalPrivateKey {
    pub x: BigInt,
}

#[derive(Debug)]
pub struct ElGamalPublicKey {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
}

impl Default for ElGamalKey {
    fn default() -> Self {
        Self { public: Default::default(), private: Default::default() }
    }
}

impl Default for ElGamalPublicKey {
    fn default() -> Self {
        Self {
            p: BigInt::zero(),
            g: BigInt::zero(),
            y: BigInt::zero(),
        }
    }
}

impl Default for ElGamalPrivateKey {
    fn default() -> Self {
        Self { x: BigInt::zero() }
    }
}

impl Savable for ElGamalPrivateKey {
    fn save(&mut self, path: String, base64_output: bool) -> Result<(), Box<dyn Error>> {
        let mut f = Self::get_file_writer(path, base64_output, "PRIVATE_KEY");
        let data = self.x.to_bytes_le().1;
        f.write_all(&(data.len() as u32).to_le_bytes()).unwrap();
        f.write_all(&self.x.to_bytes_le().1).unwrap();
        f.flush().unwrap();
        Ok(())
    }
}

impl Savable for ElGamalPublicKey {
    fn save(&mut self, path: String, base64_output: bool) -> Result<(), Box<dyn Error>> {
        let mut f = Self::get_file_writer(path, base64_output, "PUBLIC_KEY");
        let data = vec![self.p.to_bytes_le().1, self.y.to_bytes_le().1, self.g.to_bytes_le().1];
        let len = data.iter().map(|x| x.len() as u32).collect::<Vec<_>>();
        for l in len {
            f.write_all(&l.to_le_bytes()).unwrap();
        }
        for d in data {
            f.write_all(d.as_slice()).unwrap();
        }
        f.flush().unwrap();
        Ok(())
    }
}

impl Savable for ElGamalKey {
    fn save(&mut self, path: String, base64_output: bool) -> Result<(), Box<dyn Error>> {
        let path_public = path.clone() + ".pub";
        self.public.save(path_public, base64_output).unwrap();
        self.private.save(path, base64_output).unwrap();
        Ok(())
    }
}

pub trait IsEmptyKey {
    fn is_empty(&self) -> bool;
}

impl IsEmptyKey for ElGamalPublicKey {
    fn is_empty(&self) -> bool {
        self.p.is_zero() || self.y.is_zero() || self.g.is_zero()
    }
}

impl IsEmptyKey for ElGamalPrivateKey {
    fn is_empty(&self) -> bool {
        self.x.is_zero()
    }
}

impl IsEmptyKey for ElGamalKey {
    fn is_empty(&self) -> bool {
        self.private.is_empty() && self.public.is_empty()
    }
}