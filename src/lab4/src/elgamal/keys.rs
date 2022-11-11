use num_bigint::BigInt;

#[derive(Debug)]
pub struct ElGamalKey {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
    pub x: BigInt,
}