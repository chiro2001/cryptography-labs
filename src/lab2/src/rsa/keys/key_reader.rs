use std::fs::File;
use std::io::{Cursor, Read};
use crate::rsa::keys::{Key, KeyError};

impl Key {
    fn is_binary(content: &Vec<u8>) -> bool {
        content.iter().filter(|x| (**x as char).is_ascii()).count() < content.len() / 2
    }

    pub fn load(path: &String) -> Result<Key, KeyError> {
        let mut content: Vec<u8> = Vec::new();
        File::open(path).unwrap().read_to_end(&mut content).expect(format!("Unable to read file {}", path).as_str());
        let content = match Self::is_binary(&content) {
            true => content,
            false => {
                // use a cursor as the simplest possible `Read` -- in real code this is probably a file, etc.
                let mut wrapped_reader = Cursor::new(content.as_slice());
                let mut decoder = base64::read::DecoderReader::new(
                    &mut wrapped_reader, base64::STANDARD);

                // handle errors as you normally would
                let mut result = Vec::new();
                decoder.read_to_end(&mut result).unwrap();
                result
            }
        };
        if content.len() <= 8 {
            return Err(KeyError::FormatError);
        }
        let mut len_base: [u8; 4] = [0; 4];
        let mut len_m: [u8; 4] = [0; 4];
        let mut cur = Cursor::new(content);
        cur.read(&mut len_base).unwrap();
        cur.read(&mut len_m).unwrap();
        let (len_base, len_m) = (u32::from_be_bytes(len_base), u32::from_be_bytes(len_m));
        // let content_base_slice = &content[8..(8 + len_base)];
        // println!("len1: {}", content_base_slice.len());
        // let content_m_slice = &content[(8 + len_base)..(8 + len_base + len_m)];
        // println!("len2: {}", content_m_slice.len());

        // let content_base = Vec::new();
        // let mut offset = 0;
        // let a = content_base_slice.read_slice(&mut offset, len_base);
        Ok(Key { base: Default::default(), m: Default::default() })
    }
}