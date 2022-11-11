use std::error::Error;
use clap::Parser;
pub use elgamal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let fake: ElGamalFake = ElGamalFake::parse();
    let mut r: ElGamal = ElGamal::from(fake);
    if !SILENT.is_set().unwrap() { SILENT.set(r.silent).unwrap(); }
    r.run_elgamal()
}
