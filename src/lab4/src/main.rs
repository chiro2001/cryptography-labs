use std::error::Error;
use clap::Parser;
pub use elgamal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let fake: ElGamalFake = ElGamalFake::parse();
    let mut r: ElGamal = ElGamal::from(fake);
    r.run_elgamal()
}

// #[cfg(test)]
// mod tests {
//     use std::error::Error;
//     use std::io::Read;
//     use elgamal::ElGamal;
//     use elgamal::*;
//     use sha256::Sha256Digest;
//
//     fn get_stu_id_data(r: &ElGamal) -> Vec<u8> {
//         let mut reader = r.reader();
//         let mut data: [u8; 9] = [0; 9];
//         reader.read(&mut data).unwrap();
//         data.to_vec()
//     }
//
//     #[test]
//     fn test_generate_key() -> Result<(), Box<dyn Error>> {
//         let r: &ElGamal = CONFIG_DEF.get();
//         let key = r.elgamal_generate_key();
//         println!("generated key: {:#?}", key);
//         Ok(())
//     }
//
//     #[test]
//     fn test_hash_data() -> Result<(), Box<dyn Error>> {
//         let r: &ElGamal = CONFIG_DEF.get();
//         let data = get_stu_id_data(&r);
//         let val = data.digest();
//         println!("hash({}) = {}", String::from_utf8(data).unwrap(), val);
//         Ok(())
//     }
//
//     #[test]
//     fn test_sign_data() -> Result<(), Box<dyn Error>> {
//         let r: &ElGamal = CONFIG_DEF.get();
//         let data = get_stu_id_data(&r);
//         let key = r.elgamal_generate_key();
//         println!("generated key: {:#?}", key);
//         let sign = ElGamal::elgamal_sign(&data, &key);
//         println!("generated sign: {:#?}", sign);
//         Ok(())
//     }
//
//     #[test]
//     fn test_sign_check() -> Result<(), Box<dyn Error>> {
//         let r: &ElGamal = CONFIG_DEF.get();
//         let data = get_stu_id_data(&r);
//         let key = r.elgamal_generate_key();
//         println!("generated key: {:#?}", key);
//         let sign = ElGamal::elgamal_sign(&data, &key);
//         println!("generated sign: {:#?}", sign);
//         let check = ElGamal::elgamal_check(&data, &sign, &key.public);
//         assert!(check);
//         Ok(())
//     }
//
//     #[test]
//     fn test_key_save() -> Result<(), Box<dyn Error>> {
//         let r: &ElGamal = CONFIG_DEF.get();
//         let mut key = r.elgamal_generate_key();
//         key.save("data/test".to_string(), true)
//     }
// }