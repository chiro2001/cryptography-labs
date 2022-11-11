use std::error::Error;
use clap::Parser;
pub use elgamal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let fake: ElGamalFake = ElGamalFake::parse();
    let mut r: ElGamal = ElGamal::from(fake);
    r.run_elgamal()
}
