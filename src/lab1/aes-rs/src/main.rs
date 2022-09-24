use std::fs::File;
use ndarray::prelude::*;
use ndarray::Array;
use futures::executor::block_on;
use std::io;
use std::io::{Bytes, Read, Stdin, Write};
use log::{info, trace, warn};
use clap::Parser;
use std::fmt::{Display, Formatter};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "stdin", help = "Input filename")]
    input: String,
    #[clap(short, long, value_parser, default_value = "stdout", help = "Output filename")]
    output: String,
    #[clap(short, long, value_parser, default_value = "decode", value_parser = ["decode", "encode"], help = "Decode or encode data")]
    direction: String,
    #[clap(short, long, value_parser, default_value = "ECB", value_parser = ["ECB", "CBC"], help = "Run mode")]
    mode: String,
    #[clap(short, long, value_parser, default_value = "1145141919810aaa", help = "Decode / encode key")]
    key: String,
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "input={}, output={}", self.input, self.output)
    }
}

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use crate::RunMode::ECB;

#[derive(Debug)]
struct AES {
    state: Array::<u8, Ix2>,
    w: Array::<u32, Ix1>,
    round: u8,
}

#[derive(Debug)]
enum RunMode {
    ECB,
    CBC,
}

impl AES {
    fn new() -> AES {
        return AES { state: Array::<u8, Ix2>::zeros((4, 4)), w: Array::zeros(44), round: 0 };
    }
    fn encode() {}
}

// async fn run<T: Read>(stream: Bytes<T>, key: String, writer: &mut dyn Write) {
async fn run(reader: &mut dyn Read, writer: &mut dyn Write, key: &String, mode: RunMode) {
    for b in reader.bytes() {
        print!("{}", b.unwrap() as char);
    }
    writer.write_all("done\n".as_bytes()).unwrap();
}

fn main() {
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();
    let args = Args::parse();
    println!("args: {}", args);
    // let a = Array::<u8, Ix2>::zeros((4, 4));
    // println!("a: {}", a);
    // // let a = Array::range(0., 10., 1.);
    //
    // let mut a = a.mapv(|a: u8| (a + 2) * (a + 1));  // numpy equivlant of `a ** 3`; https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.powi
    //
    // println!("{}", a);
    //
    // println!("{}", a[[2, 2]]);
    //
    // let mut state = AES::new();
    // state.state = a;
    // println!("state: {:?}", state);
    //
    // // let file = File::open("test.txt")?;
    // // file.and_then(|s| println!("{}", s)).await;
    //
    // // let iter = File::open("stdin")?.bytes().chain().into_iter();
    // // println!("{}", iter);
    //
    // let key = String::from("securitysecurity");
    //
    // let mut writer = File::create("res.aes").unwrap();
    // writer.write_all("ss".as_bytes()).unwrap();
    //
    // // block_on(run(io::stdin().bytes()));
    // // block_on(run(File::open("xmake.lua").unwrap().bytes(), key, &mut writer));

    let key = String::from("securitysecurity");
    let mut reader: Box<dyn Read> = Box::new(io::stdin());
    let mut writer: Box<dyn Write> = Box::new(io::stdout());
    block_on(run(&mut reader, &mut writer, &key, ECB));

    // let mut reader: dyn Read = match args.input.as_str() {
    //     "stdin" => io::stdin(),
    //     i => File::open(i).unwrap()
    // };
    // let mut writer: dyn Write = match args.output.as_str() {
    //     "stdout" => io::stdout(),
    //     o => File::create(o).unwrap()
    // };
    // block_on(run(&mut reader, &mut writer, &args.key, ECB));
    // println!("input: {}", input);
}
