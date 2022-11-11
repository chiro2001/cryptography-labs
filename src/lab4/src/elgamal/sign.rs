use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};
use num_bigint::{BigInt, Sign};
use num_traits::Zero;
use rsa::keys::KeyReader;
use crate::Savable;

#[derive(Debug, PartialEq)]
pub struct ElGamalSign {
    pub r: BigInt,
    pub s: BigInt,
}

impl Default for ElGamalSign {
    fn default() -> Self {
        ElGamalSign { r: BigInt::zero(), s: BigInt::zero() }
    }
}

impl Savable for ElGamalSign {
    fn save(&mut self, path: String, base64_output: bool) -> Result<(), Box<dyn Error>> {
        let mut f = Self::get_file_writer(path, base64_output, "FILE_SIGN");
        let data = vec![self.r.to_bytes_le().1, self.s.to_bytes_le().1];
        let len = data.iter().map(|x| x.len() as u32).collect::<Vec<_>>();
        for l in &len {
            f.write_all(&l.to_le_bytes()).unwrap();
        }
        for d in data {
            f.write_all(d.as_slice()).unwrap();
        }
        f.flush().unwrap();
        println!("Sign save bytes: {} : {} + {}", len.iter().sum::<u32>(), self.r.to_bytes_le().1.len(), self.s.to_bytes_le().1.len());
        Ok(())
    }
}

impl From<String> for ElGamalSign {
    fn from(path: String) -> Self {
        let file = File::open(path);
        match file {
            Err(_) => return ElGamalSign::default(),
            _ => {}
        };
        let mut key_reader = KeyReader::new(Box::new(file.unwrap()));
        let content = key_reader.read_all();
        let mut cur = Cursor::new(&content);
        let mut len_r: [u8; 4] = [0; 4];
        let mut len_s: [u8; 4] = [0; 4];
        for l in vec![&mut len_r, &mut len_s] {
            cur.read(l).unwrap();
        }
        let (len_r, len_s) = (u32::from_le_bytes(len_r), u32::from_le_bytes(len_s));
        let mut data = Vec::new();
        cur.read_to_end(&mut data).unwrap();

        assert_eq!(len_r + len_s, data.len() as u32,
                   "Sign format err: sign lens: ({}, {}), data len {}", len_r, len_s, data.len());

        let (mut r, mut s) = (Vec::new(), Vec::new());
        let mut index = 0;
        for _ in 0..len_r {
            r.push(data[index]);
            index += 1;
        }
        for _ in 0..len_s {
            s.push(data[index]);
            index += 1;
        }
        let (r, s) = (
            BigInt::from_bytes_le(Sign::Plus, r.as_slice()),
            BigInt::from_bytes_le(Sign::Plus, s.as_slice()));
        ElGamalSign { r, s }
    }
}