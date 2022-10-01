pub mod key_writer;
pub mod key_reader;
pub mod key_data;
pub mod key_pair;

use num_bigint::BigInt;
use crate::rsa::keys::key_data::KeyData;

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

const BASE64_SPLIT: usize = 70;
