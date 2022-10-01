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