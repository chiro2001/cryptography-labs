pub mod config {
    use clap::Parser;
    use lazy_static::lazy_static;
    use mut_static::MutStatic;

    #[derive(Debug, Parser, Copy, Clone)]
    pub struct Config {
        #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_min, help = "Min prime bits")]
        pub prime_min: u32,
        #[clap(long, value_parser, required = false, default_value_t = CONFIG_DEF.prime_max, help = "Max prime bits")]
        pub prime_max: u32,
    }

    impl Config {
        pub fn get(&self) -> &Config {
            self
        }
        pub fn set(&mut self, other: Config) {
            *self = other;
        }
    }

    lazy_static! {
            pub static ref CONFIG_DEF: Config = Config { prime_min: 14, prime_max: 1024 };
            pub static ref CONFIG: MutStatic<Config> = {
                MutStatic::new()
            };
        }
}