use std::fs::File;
use ndarray::prelude::*;
use ndarray::Array;
use futures::executor::block_on;
use std::io;
use std::io::{Bytes, Read, Stdin};
use log::{info, trace, warn};

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

#[derive(Debug)]
struct AES {
    state: Array::<u8, Ix2>,
    w: Array::<u32, Ix1>,
    round: u8,
}

impl AES {
    fn new() -> AES {
        return AES { state: Array::<u8, Ix2>::zeros((4, 4)), w: Array::zeros(44), round: 0 };
    }
    fn encode() {}
}

async fn run<T: Read>(stream: Bytes<T>, key: String) {
    trace!("a trace log");
    info!("a info long: {}", "abc");

    info!("{}:{}", file!(), line!());

    error!("Bright red error");
    info!("This only appears in the log file");
    debug!("This level is currently not enabled for any logger");
    // warn!("a warning log: {}, retrying", err);
    for b in stream {
        print!("{}", b.unwrap() as char);
    }
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
    println!("AES-rs!");
    let a = Array::<u8, Ix2>::zeros((4, 4));
    println!("a: {}", a);
    // let a = Array::range(0., 10., 1.);

    let mut a = a.mapv(|a: u8| (a + 2) * (a + 1));  // numpy equivlant of `a ** 3`; https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.powi

    println!("{}", a);

    println!("{}", a[[2, 2]]);

    let mut state = AES::new();
    state.state = a;
    println!("state: {:?}", state);

    // let file = File::open("test.txt")?;
    // file.and_then(|s| println!("{}", s)).await;

    // let iter = File::open("stdin")?.bytes().chain().into_iter();
    // println!("{}", iter);

    let key = String::from("securitysecurity");

    // block_on(run(io::stdin().bytes()));
    block_on(run(File::open("xmake.lua").unwrap().bytes(), key));
}
