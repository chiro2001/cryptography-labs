use num_bigint::BigInt;

#[derive(Debug)]
pub struct ElGamalSign {
    pub r: BigInt,
    pub s: BigInt,
}