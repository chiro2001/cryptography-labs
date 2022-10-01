pub mod key_writer;
pub mod key_reader;
pub mod key_data;
pub mod key_pair;

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

const BASE64_SPLIT: usize = 70;
