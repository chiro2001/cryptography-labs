mod aes_rs;

use std::error::Error;
use std::fs::File;
use futures::executor::block_on;
use std::io;
use std::io::{Read, Write};
use log::{info};
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::iter::zip;
use simplelog::*;
use crate::aes_rs::aes_rs::{AES, RunMode};

extern crate lazy_static;
extern crate log;
extern crate simplelog;

#[derive(Debug, Parser)]
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
        write!(f, "input={}, output={}, direction={}, mode={}, key={}", self.input, self.output, self.direction, self.mode, self.key)
    }
}

pub async fn run(reader: &mut dyn Read, writer: &mut dyn Write, key: &String, mode: RunMode, encode: bool) {
    let mut keys = [0 as u8; 16];
    for (a, b) in zip(keys.iter_mut(), key.as_bytes()) { *a = *b; }
    let mut aes = AES::new(keys, mode);
    if encode { aes.encode(reader, writer).await; } else { aes.decode(reader, writer).await; }
}

fn main() -> Result<(), Box<dyn Error>> {
    CombinedLogger::init(vec![TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)]).unwrap();
    let args = Args::parse();
    if args.output != "stdout" { info!("args: {}", args); }

    let mut reader: Box<dyn Read> = match args.input.as_str() {
        "stdin" => Box::new(io::stdin()),
        f => Box::new(File::open(f).unwrap())
    };
    let mut writer: Box<dyn Write> = match args.output.as_str() {
        "stdout" => Box::new(io::stdout()),
        f => Box::new(File::create(f).unwrap())
    };
    let mode = match args.mode.as_str() {
        "ECB" => Ok(RunMode::ECB),
        "CBC" => Ok(RunMode::CBC),
        _ => Err(())
    }.unwrap();
    if args.direction == "both" {
        let mut stdout: Box<dyn Write> = Box::new(io::stdout());
        block_on(run(&mut reader, &mut writer, &args.key, mode, true));
        assert!(args.output.as_str() != "stdout");
        let mut encoded = Box::new(File::open(args.output.as_str()).unwrap());
        block_on(run(&mut encoded, &mut stdout, &args.key, mode, false));
    } else {
        block_on(run(&mut reader, &mut writer, &args.key, mode, args.direction == "encode"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use ndarray::prelude::*;
    use ndarray::{array, concatenate, s, stack};
    use crate::{AES, RunMode};

    #[test]
    fn array_test() {
        println!("testing ndarray");

        let a: Array<u8, Ix2> = array![
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

    fn init_matrix() -> [u8; 16] {
        let mut v = Vec::new();
        for i in 0..16 {
            v.push(i as u8);
        }
        // let m = Array::from(v).into_shape((4, 4)).unwrap();
        let mut m = [0 as u8; 16];
        let _ = m.iter_mut().zip(v).map(|x| *x.0 = x.1).collect::<Vec<_>>();
        m
    }

    #[test]
    fn function_test() {
        let key = String::from("securitysecurity");
        let mut bytes = [0 as u8; 16];
        let _ = bytes.iter_mut().zip(key.as_bytes()).map(|x| *x.0 = *x.1).collect::<Vec<_>>();
        let mut aes = AES::new(bytes, RunMode::ECB);
        let a = init_matrix();
        println!("a: {a:x?}");
        aes.state = a;
        aes.sub_bytes();
        println!("sub: {:x?}", aes.state);
        println!("T(0, 0): {:x}", AES::function_t(0, 0));

        aes.state = init_matrix();
        aes.mix_columns();
        println!("mix:\n{:3x?}", aes.state);
        aes.mix_columns_inv();
        println!("mix_inv:\n{:3x?}", aes.state);

        aes.state = init_matrix();
        aes.shift_rows();
        println!("shift:\n{:3x?}", aes.state);
        aes.shift_rows_inv();
        println!("shift_inv:\n{:3x?}", aes.state);

        println!("gf_mul(1, 1): {:x}", AES::gf_mul(1, 1));
        println!("gf_mul2(1): {:x}", AES::gf_mul2(1));
    }
}