use std::fs::File;
use std::io::{Cursor, Read, Write};
use crate::rsa::keys::{BASE64_SPLIT, Key, KEY_PRIVATE, KEY_PUBLIC, KeyError, KeyPrivate, KeyPublic, KeyTypeName};

struct KeyWriter<T: KeyTypeName> {
    writer: Box<dyn Write>,
    buffer: Vec<u8>,
    header: String,
    footer: String,
    mode: T,
}

impl From<Box<(dyn Write + 'static)>> for KeyWriter<KeyPrivate> {
    fn from(f: Box<(dyn Write + 'static)>) -> Self {
        Self::new(f, KEY_PRIVATE)
    }
}

impl From<File> for KeyWriter<KeyPrivate> {
    fn from(f: File) -> Self {
        Self::new(Box::new(f), KEY_PRIVATE)
    }
}

impl From<Box<File>> for KeyWriter<KeyPrivate> {
    fn from(f: Box<File>) -> Self {
        Self::new(f, KEY_PRIVATE)
    }
}

impl From<Box<(dyn Write + 'static)>> for KeyWriter<KeyPublic> {
    fn from(f: Box<(dyn Write + 'static)>) -> Self {
        Self::new(f, KEY_PUBLIC)
    }
}

impl From<File> for KeyWriter<KeyPublic> {
    fn from(f: File) -> Self {
        Self::new(Box::new(f), KEY_PUBLIC)
    }
}

impl From<Box<File>> for KeyWriter<KeyPublic> {
    fn from(f: Box<File>) -> Self {
        Self::new(f, KEY_PUBLIC)
    }
}

impl<T: KeyTypeName> KeyWriter<T> {
    pub fn new(f: Box<dyn Write>, mode: T) -> Self {
        KeyWriter {
            writer: f,
            buffer: vec![],
            header: "-----BEGIN RSA-RS {} KEY-----".to_string(),
            footer: "-----END RSA-RS {} KEY-----".to_string(),
            mode,
        }
    }
}

impl<T: KeyTypeName> Write for KeyWriter<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for b in buf { self.buffer.push(*b); }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut cur = Cursor::new(&self.buffer);
        self.writer.write_all((self.header.replace("{}", &T::get_type_name())).as_bytes()).unwrap();
        self.writer.write_all("\n".as_bytes()).unwrap();
        loop {
            let mut buf: [u8; BASE64_SPLIT] = [0; BASE64_SPLIT];
            let n = cur.read(&mut buf);
            match n {
                Ok(0) => break,
                Err(_) => break,
                _ => {}
            }
            let n = n.unwrap();
            self.writer.write_all(&buf[0..n]).unwrap();
            self.writer.write_all("\n".as_bytes()).unwrap();
        }
        self.writer.write_all((self.footer.replace("{}", &T::get_type_name())).as_bytes()).unwrap();
        self.writer.flush()
    }
}

impl Key {
    pub fn save(&self, path: &String, base64_output: bool) -> Result<(), KeyError> {
        let base = self.base.to_bytes_le().1;
        let m = self.m.to_bytes_le().1;
        if base.len() < 8 || m.len() < 8 {
            return Err(KeyError::FormatError);
        }
        let mut f: Box<dyn Write> = match base64_output {
            true => Box::new(base64::write::EncoderWriter::new(
                KeyWriter::<KeyPrivate>::from(Box::new(File::create(path).unwrap())),
                base64::STANDARD)),
            false => Box::new(File::create(path).unwrap())
        };
        let lens: [u32; 2] = [base.len() as u32, m.len() as u32];
        f.write_all(&lens[0].to_le_bytes()).unwrap();
        f.write_all(&lens[1].to_le_bytes()).unwrap();
        f.write_all(base.as_slice()).unwrap();
        f.write_all(m.as_slice()).unwrap();
        f.flush().unwrap();
        Ok(())
    }
}