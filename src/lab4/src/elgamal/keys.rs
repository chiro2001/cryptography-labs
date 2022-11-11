use num_bigint::BigInt;

#[derive(Debug)]
pub struct ElGamalKey {
    pub public: ElGamalPublicKey,
    pub private: ElGamalPrivateKey,
}

#[derive(Debug)]
pub struct ElGamalPublicKey {
    pub p: BigInt,
    pub g: BigInt,
    pub y: BigInt,
}

#[derive(Debug)]
pub struct ElGamalPrivateKey {
    pub x: BigInt,
}
