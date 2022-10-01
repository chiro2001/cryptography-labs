use lazy_static::lazy_static;
use mut_static::MutStatic;
use crate::rsa::RSA;

lazy_static! {
    pub static ref CONFIG_DEF: RSA = RSA {
        mode: String::from("generate"),
        key: String::from("data/rsa"),
        input: String::from("data/lab2-Plaintext.txt"),
        // output: String::from("stdout"),
        output: String::from("data/data.tmp"),
        prime_min: 14, prime_max: 512,
        base64: false,
        rounds: 10,
        time_max: 1000,
        silent: false,
        threads: 8,
        retry: true,
        comment: String::from("RSA-RS COMMENT")
    };
    pub static ref SILENT: MutStatic<bool> = MutStatic::from(false);
}
