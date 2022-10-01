use std::borrow::Borrow;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};
use crate::rsa::keys::{Key, KEY_PUBLIC, KeyError, KeyTypeName};

struct KeyReader<T: KeyTypeName> {
    reader: Box<dyn Read>,
    mode: T,
    binary: Option<bool>,
    temp: [u8; 8],
    read_buf: Vec<u8>,
    res_buf: Vec<u8>,
    cur: u64,
}

impl<T: KeyTypeName> KeyReader<T> {
    pub fn new(reader: Box<dyn Read>, mode: T) -> Self {
        let mut s = Self { reader, mode, binary: None, temp: [0; 8], read_buf: vec![], res_buf: vec![], cur: 0 };
        s.judge_binary().unwrap();
        s.parse_text().unwrap();
        println!("res_buf: {:x?}", s.res_buf);
        println!("res: {}", String::from_utf8(s.res_buf.clone()).unwrap());
        s
    }

    fn parse_text(&mut self) -> Result<(), KeyError> {
        let mut cur = Cursor::new(&self.read_buf);
        let mut line = String::new();
        while let Ok(n) = cur.read_line(&mut line) {
            println!("line: {}", line);
            if n > 0 {
                if !line.starts_with("-") {
                    for c in line.as_bytes() {
                        if *c != '\n' as u8 { self.res_buf.push(*c); }
                    }
                }
            } else { break; }
            line.clear();
        }
        // self.cur = Some(Cursor::new(self.res_buf.clone()));
        Ok(())
    }

    fn judge_binary(&mut self) -> Result<(), KeyError> {
        if self.binary.is_none() {
            match self.reader.read(&mut self.temp) {
                Ok(n) => match n {
                    8 => {
                        let count = self.temp.iter().filter(|x| x.is_ascii_graphic()).count();
                        println!("count: {}, data: {}", count, String::from_utf8(self.temp.to_vec()).unwrap());
                        self.binary = Some(count < 8);
                        for t in self.temp { self.read_buf.push(t); }
                        self.reader.read_to_end(&mut self.read_buf).unwrap();
                        Ok(())
                    }
                    _ => Err(KeyError::FormatError)
                },
                _ => Err(KeyError::FormatError)
            }
        } else {
            Ok(())
        }
    }
}

impl<T: KeyTypeName> Read for KeyReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.binary {
            Some(true) => self.reader.read(buf),
            Some(false) => {
                let mut reader = Cursor::new(&self.res_buf);
                reader.seek(SeekFrom::Start(self.cur)).unwrap();
                let res = reader.read(buf);
                match &res {
                    Ok(n) => self.cur += *n as u64,
                    _ => {}
                }
                res
            }
            None => panic!("Call `self.judge_binary()' first!")
        }
    }
}

impl Key {
    fn is_binary(content: &Vec<u8>) -> bool {
        content.iter().filter(|x| (**x as char).is_ascii()).count() < content.len() / 2
    }

    pub fn load(path: &String) -> Result<Key, KeyError> {
        let mut content: Vec<u8> = Vec::new();
        File::open(path).unwrap().read_to_end(&mut content).expect(format!("Unable to read file {}", path).as_str());
        let reader = KeyReader::new(Box::new(File::open("data/test.pub").unwrap()), KEY_PUBLIC);
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

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs::File;
    use std::io::Read;
    use crate::rsa::keys::{Key, KEY_PUBLIC};
    use crate::rsa::keys::key_reader::KeyReader;

    #[test]
    fn test_binary() -> Result<(), Box<dyn Error>> {
        let reader = KeyReader::new(Box::new(File::open("build/linux/x86_64/release/rsa").unwrap()), KEY_PUBLIC);
        println!("binary: {:?}", reader.binary);
        let reader = KeyReader::new(Box::new(File::open("data/test.pub").unwrap()), KEY_PUBLIC);
        println!("binary: {:?}", reader.binary);
        Ok(())
    }

    #[test]
    fn test_base64() -> Result<(), Box<dyn Error>> {
        let mut reader = KeyReader::new(Box::new(File::open("data/test.pub").unwrap()), KEY_PUBLIC);
        println!("binary: {:?}", reader.binary);
        let mut reader = base64::read::DecoderReader::new(&mut reader, base64::STANDARD);
        let mut res = Vec::new();
        reader.read_to_end(&mut res).unwrap();
        println!("res: {:x?}", res);
        Ok(())
    }
}
