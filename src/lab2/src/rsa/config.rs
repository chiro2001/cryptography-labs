use lazy_static::lazy_static;
use mut_static::MutStatic;
use crate::rsa::RSA;

lazy_static! {
    pub static ref CONFIG_DEF: RSA = RSA {
        prime_min: 14, prime_max: 512,
        input: String::from("data/lab2-Plaintext.txt"),
        // output: String::from("stdout"),
        output: String::from("data/data.tmp"),
        base64_out: true,
        base64_in: false,
        rounds: 10,
        time_max: 1000,
        mode: String::from("generate"),
        silent: false,
        key_public: String::from("~/.ssh/id_rsa.pub"),
        key_private: String::from("~/.ssh/id_rsa"),
        threads: 8,
        retry: true
    };
    pub static ref SILENT: MutStatic<bool> = MutStatic::from(false);
}
