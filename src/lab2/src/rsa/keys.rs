use std::fs::File;
use std::io::{Cursor, Read, Write};
use num_bigint::BigInt;

#[derive(Debug)]
pub struct Key {
    pub base: BigInt,
    pub m: BigInt,
}

#[derive(Debug)]
pub struct KeySet {
    pub public: Key,
    pub private: Key,
}

#[derive(Debug)]
pub enum KeyError {
    ParseError(String),
    FormatError,
}

impl From<String> for Key {
    fn from(path: String) -> Self {
        Self::load(&path).unwrap()
    }
}

impl From<&str> for Key {
    fn from(path: &str) -> Self {
        Self::load(&path.to_string()).unwrap()
    }
}

struct KeyPrivate;

struct KeyPublic;

const KEY_PRIVATE: KeyPrivate = KeyPrivate {};
const KEY_PUBLIC: KeyPublic = KeyPublic {};

trait KeyTypeName {
    fn get_type_name() -> String;
}

impl KeyTypeName for KeyPublic {
    fn get_type_name() -> String { "PUBLIC".to_string() }
}

impl KeyTypeName for KeyPrivate {
    fn get_type_name() -> String { "PRIVATE".to_string() }
}

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

const BASE64_SPLIT: usize = 70;

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

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::str::FromStr;
    use num_bigint::BigInt;
    use crate::rsa::keys::Key;

    #[test]
    fn test_key_save() -> Result<(), Box<dyn Error>> {
        // get keys: KeySet { public: Key {
        // base: 1918022387998207002219685790885422000751305870471197615474028403267680489056093224556537201212850361018884250661472831998657776705844077320563372556461358381610646614987785380742299336150827724805642511244842883909329821830379863046595720068818656861135676253679842005301808669637130750097414684421703920179,
        // m: 3109289493004396232718677839075724040902511898079934814878402066495223327499893476355898158210192836071346652507223581255051247419447099538963654140505175239054593601039582793123484358359757335255732592975546398804612740515822807396398780156996203878672950754579450708664916820774253607630761518662668511457 },
        // private: Key {
        // base: 1647244904834020890590328220079077450255926221297431015846481670679976278867195022845440259910374906689286200978495387347099819640456100034631096132217180522459428379766721029286016492809874734755016913514610957434891297109301215308348746352680481967531502218226057761620024021288408693507383518495475314247,
        // m: 3109289493004396232718677839075724040902511898079934814878402066495223327499893476355898158210192836071346652507223581255051247419447099538963654140505175239054593601039582793123484358359757335255732592975546398804612740515822807396398780156996203878672950754579450708664916820774253607630761518662668511457 } }
        let key = Key {
            base: BigInt::from_str("1918022387998207002219685790885422000751305870471197615474028403267680489056093224556537201212850361018884250661472831998657776705844077320563372556461358381610646614987785380742299336150827724805642511244842883909329821830379863046595720068818656861135676253679842005301808669637130750097414684421703920179").unwrap(),
            m: BigInt::from_str("3109289493004396232718677839075724040902511898079934814878402066495223327499893476355898158210192836071346652507223581255051247419447099538963654140505175239054593601039582793123484358359757335255732592975546398804612740515822807396398780156996203878672950754579450708664916820774253607630761518662668511457").unwrap(),
        };
        key.save(&"data/test.pub".to_string(), true).unwrap();
        Ok(())
    }

    #[test]
    fn test_key_load() -> Result<(), Box<dyn Error>> {
        // let key = Key::new()
        Ok(())
    }
}