mod aes_rs;

use std::fs::File;
use futures::executor::block_on;
use std::io;
use std::io::{Read, Write};
use log::{info};
use clap::Parser;
use std::fmt::{Display, Formatter};
use simplelog::*;
use crate::aes_rs::aes_rs::{AES, RunMode};

extern crate lazy_static;
extern crate log;
extern crate simplelog;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "stdin", help = "Input filename")]
    input: String,
    #[clap(short, long, value_parser, default_value = "stdout", help = "Output filename")]
    output: String,
    #[clap(short, long, value_parser, default_value = "encode", value_parser = ["decode", "encode", "both"], help = "Decode or encode data")]
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

pub async fn run(reader: &mut dyn Read, writer: &mut dyn Write, key: &String, mode: RunMode, encode: bool) {
    let mut aes = AES::new(key, mode);
    if encode { aes.encode(reader, writer).await; } else { aes.decode(reader, writer).await; }
}

fn main() {
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();
    let args = Args::parse();
    info!("args: {}", args);

    let mut reader: Box<dyn Read> = match args.input.as_str() {
        "stdin" => Box::new(io::stdin()),
        f => Box::new(File::open(f).unwrap())
    };
    let mut writer: Box<dyn Write> = match args.output.as_str() {
        "stdout" => Box::new(io::stdout()),
        f => Box::new(File::create(f).unwrap())
    };
    if args.direction == "both" {
        let mut stdout: Box<dyn Write> = Box::new(io::stdout());
        block_on(run(&mut reader, &mut writer, &args.key, RunMode::ECB, true));
        block_on(run(&mut reader, &mut stdout, &args.key, RunMode::ECB, false));
    } else {
        block_on(run(&mut reader, &mut writer, &args.key, RunMode::ECB, args.direction == "encode"));
    }
}

mod tests {
    #[cfg(test)]
    use std::io::Write;
    use ndarray::prelude::*;
    use ndarray::{array, concatenate, s, stack};
    use crate::AES;

    #[test]
    fn array_test() {
        println!("testing ndarray");

        let mut a: Array<u8, Ix2> = array![
            [0, 0, 1, 4],
            [2, 3, 4, 4],
            [5, 5, 6, 4],
            [5, 5, 6, 4]
        ];
        println!("{}", (0..4).map(|i| i * i).sum::<u8>());
        println!("{}", (0..4).map(|i| i * i).fold(0, |a, b| a + b));
        println!("{}", (0..4).map(|i| array![i, i, i, i])
            .reduce(|a, b| concatenate![Axis(0), a, b])
            .unwrap().into_shape((4, 4)).unwrap());

        let stacks = (0..4).map(|i| {
            let r = a.slice(s![i, ..]);
            let c = concatenate![Axis(0), r, r];
            Array::from(c.slice(s![i..(i+4)]).to_vec())
        }).reduce(|a, b| concatenate![Axis(0), a, b]).unwrap().into_shape((4, 4)).unwrap();
        println!("stacks: {}", stacks);


        let s = stack![Axis(0), array![0, 0, 0], array![1, 2, 2], array![0, 0, 0]];
        println!("s: {}", s);
        // println!("stacks: {}", stacks[0]);
    }

    #[test]
    fn function_test() {
        let mut v = Vec::new();
        for i in 0..16 {
            v.push(i);
        }
        let a = Array::from(v).into_shape((4, 4)).unwrap();
        println!("a: {a:x}");
        let mut aes = AES::new();
    }
}