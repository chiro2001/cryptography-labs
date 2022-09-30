pub mod config {
    use std::error::Error;
    use clap::Parser;
    use lazy_static::lazy_static;
    use mut_static::MutStatic;

    #[derive(Debug, Parser)]
    pub struct Config {
        #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_min, help = "Min prime bits")]
        pub prime_min: u32,
        #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_max, help = "Max prime bits")]
        pub prime_max: u32,
        #[clap(short, long, value_parser, default_value = CONFIG_DEF.input.as_str(), help = "Input filename")]
        pub input: String,
        #[clap(short, long, value_parser, default_value = CONFIG_DEF.output.as_str(), help = "Output filename")]
        pub output: String,
        #[clap(long, value_parser, default_value_t = CONFIG_DEF.base64_out, help = "Output in base64 format")]
        pub base64_out: bool,
        #[clap(long, value_parser, default_value_t = CONFIG_DEF.base64_in, help = "Input in base64 format")]
        pub base64_in: bool,
        #[clap(short, long, value_parser, default_value_t = CONFIG_DEF.rounds, help = "Miller Rabin calculate rounds")]
        pub rounds: u32,
        #[clap(short, long, value_parser, default_value_t = CONFIG_DEF.time_max, help = "Max time in mill seconds that trying to generate a prime")]
        pub time_max: u32,
    }

    impl Config {
        pub fn get(&self) -> &Config {
            self
        }
        pub fn copy(&self) -> Config {
            Config {
                prime_min: self.prime_min,
                prime_max: self.prime_max,
                input: self.input.clone(),
                output: self.output.clone(),
                base64_out: self.base64_out,
                base64_in: self.base64_in,
                rounds: self.rounds,
                time_max: self.time_max
            }
        }
        pub fn set(&mut self, other: Config) {
            *self = other;
        }
    }

    lazy_static! {
        pub static ref CONFIG_DEF: Config = Config {
            prime_min: 14, prime_max: 1024,
            input: String::from("data/lab2-Plaintext.txt"),
            output: String::from("stdout"),
            base64_out: true,
            base64_in: false,
            rounds: 10,
            time_max: 5000
        };
        pub static ref CONFIG: MutStatic<Config> = MutStatic::new();
    }

    pub fn use_default() -> Result<(), Box<dyn Error>> {
        if !CONFIG.is_set().unwrap() {
            CONFIG.set(CONFIG_DEF.copy())?;
        }
        println!("Use default config: {:?}", CONFIG.read()?.get());
        Ok(())
    }
}