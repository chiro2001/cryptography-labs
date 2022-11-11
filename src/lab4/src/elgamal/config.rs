// pub use rsa::config::CONFIG_DEF;
pub use rsa::config::SILENT;
use lazy_static::lazy_static;
use num_cpus;
use crate::ElGamal;

lazy_static! {
    pub static ref CONFIG_DEF: ElGamal = ElGamal {
        mode: String::from("generate"),
        key: String::from("key"),
        input: String::from("stdin"),
        // input: String::from("data/lab2-Plaintext.txt"),
        output: String::from("stdout"),
        // output: String::from("data/data.tmp"),
        prime_min: 14, prime_max: 16,
        binary: false,
        rounds: 10,
        time_max: 1000,
        silent: false,
        threads: num_cpus::get(),
        retry: true,
        comment: String::from("RSA-RS COMMENT")
    };
}
