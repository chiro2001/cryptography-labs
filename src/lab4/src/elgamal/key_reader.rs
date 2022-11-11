use std::fs::File;
use std::io;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};
use num_bigint::{BigInt, Sign};
use rsa::keys::{KeyError, KeyReader};
use crate::ElGamalPublicKey;


impl From<String> for ElGamalPublicKey {
    fn from(path: String) -> Self {
        let file = File::open(path);
        match file {
            Err(_) => return ElGamalPublicKey::default(),
            _ => {}
        };
        let mut key_reader = KeyReader::new(Box::new(file.unwrap()));
        let mut content = key_reader.read_all();
        let mut cur = Cursor::new(&content);
        let mut len_p: [u8; 4] = [0; 4];
        let mut len_g: [u8; 4] = [0; 4];
        let mut len_y: [u8; 4] = [0; 4];
        for mut l in vec![len_p, len_g, len_y] {
            cur.read(&mut l).unwrap();
        }
        let (len_p, len_g, len_y) = (u32::from_le_bytes(len_p), u32::from_le_bytes(len_g), u32::from_le_bytes(len_y));
        let mut data = Vec::new();
        cur.read_to_end(&mut data).unwrap();

        let (mut p, mut g, mut y) = (Vec::new(), Vec::new(), Vec::new());
        let mut index = 0;
        for _ in 0..len_p {
            &p.push(data[index]);
            index += 1;
        }
        for _ in 0..len_g {
            &g.push(data[index]);
            index += 1;
        }
        for _ in 0..len_y {
            &y.push(data[index]);
            index += 1;
        }
        let (p, g, y) = (
            BigInt::from_bytes_le(Sign::Plus, p.as_slice()),
            BigInt::from_bytes_le(Sign::Plus, g.as_slice()),
            BigInt::from_bytes_le(Sign::Plus, y.as_slice()));
        ElGamalPublicKey { p, g, y }
    }
}