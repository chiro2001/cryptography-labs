use std::fs::File;
use ndarray::prelude::*;
use ndarray::{Array, concatenate};
use futures::executor::block_on;
use std::io;
use std::io::{Read, Write};
use log::{info, trace, warn};
use clap::Parser;
use std::fmt::{Display, Formatter};
use lazy_static::lazy_static;

extern crate lazy_static;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value = "stdin", help = "Input filename")]
    input: String,
    #[clap(short, long, value_parser, default_value = "stdout", help = "Output filename")]
    output: String,
    #[clap(short, long, value_parser, default_value = "encode", value_parser = ["decode", "encode"], help = "Decode or encode data")]
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
    mode: RunMode,
    key: String,
}

#[derive(Debug)]
enum RunMode {
    ECB,
    CBC,
}

lazy_static! {
    static ref S: [u8; 16 * 16] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b,
        0xfe, 0xd7, 0xab, 0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0,
        0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26,
        0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
        0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0,
        0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed,
        0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
        0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f,
        0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec,
        0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
        0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14,
        0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c,
        0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
        0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f,
        0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e,
        0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1, 0xf8, 0x98, 0x11,
        0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f,
        0xb0, 0x54, 0xbb, 0x16];
    static ref S2: [u8; 16 * 16] = [
        0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e,
        0x81, 0xf3, 0xd7, 0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87,
        0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32,
        0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
        0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
        0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16,
        0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50,
        0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
        0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05,
        0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
        0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41,
        0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
        0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8,
        0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89,
        0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
        0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
        0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59,
        0x27, 0x80, 0xec, 0x5f, 0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d,
        0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0, 0xe0, 0x3b, 0x4d,
        0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
        0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63,
        0x55, 0x21, 0x0c, 0x7d];
    static ref COL_M: Array<u8, Ix2> = array![[2, 3, 1, 1, 1, 2, 3, 1, 1, 1, 2, 3, 3, 1, 1, 2]].into_shape((4, 4)).unwrap();
    static ref COL_M_INV: Array<u8, Ix2> = array![[0xe, 0xb, 0xd, 0x9, 0x9, 0xe, 0xb, 0xd, 0xd, 0x9, 0xe, 0xb, 0xb, 0xd, 0x9, 0xe]].into_shape((4, 4)).unwrap();
    static ref RCON: [u32; 11] = [
        0x00000000, 0x01000000, 0x02000000, 0x04000000, 0x08000000,
        0x10000000, 0x20000000, 0x40000000, 0x80000000, 0x1b000000, 0x36000000];
}

impl AES {
    pub fn new(key: &String, mode: RunMode) -> AES {
        return AES { state: Array::<u8, Ix2>::zeros((4, 4)), w: Array::zeros(44), round: 0, mode, key: key.clone() };
    }

    fn add_round_key(&mut self, round: usize) {
        for i in 0..4 {
            for j in 0..4 {
                self.state[[j, i]] ^= ((self.w[[round * 4 + i]] >> ((3 - j) * 8)) & 0xff) as u8;
            }
        }
    }

    fn sub_bytes(&mut self) {
        self.state = self.state.map(|x| S[*x as usize]);
    }

    fn sub_bytes_inv(&mut self) {
        self.state = self.state.map(|x| S2[*x as usize]);
    }

    fn gf_mul2(&self, s: u8) -> u8 {
        if s & 0x80 != 0 {
            (s << 1) ^ 0x1b
        } else {
            s << 1
        }
    }

    fn gf_mul(&self, n: u8, s: u8) -> u8 {
        let mut m = n;
        let mut sum = s;
        let mut result: u8 = 0;
        while m != 0 {
            if m & 0x1 == 0 {
                result ^= sum;
            }
            m >>= 1;
            sum = self.gf_mul2(sum);
        }
        return result;
    }

    fn shift_rows(&mut self) {
        self.state = (0..4).map(|i| {
            let r = self.state.slice(s![i, ..]);
            let c = concatenate![Axis(0), r, r];
            Array::from(c.slice(s![i..(i+4)]).to_vec())
        }).reduce(|a, b| concatenate![Axis(0), a, b]).unwrap().into_shape((4, 4)).unwrap();
    }

    fn shift_rows_inv(&mut self) {
        self.state = (0..4).map(|i| {
            let r = self.state.slice(s![i, ..]);
            let c = concatenate![Axis(0), r, r];
            Array::from(c.slice(s![(4-i)..(8-i)]).to_vec())
        }).reduce(|a, b| concatenate![Axis(0), a, b]).unwrap().into_shape((4, 4)).unwrap();
    }

    fn mat_gf_mul(&mut self, m: &Array<u8, Ix2>) {
        for i in 0..4 {
            for j in 0..4 {
                self.state[[i, j]] = ((0..4).map(|k| {
                    self.gf_mul(m[[i, k]], self.state[[k, j]]) as u32
                }).sum::<u32>() & 0xff) as u8;
            }
        }
    }

    fn mix_columns(&mut self) {
        self.mat_gf_mul(&COL_M);
    }

    fn mix_columns_inv(&mut self) {
        self.mat_gf_mul(&COL_M_INV);
    }

    fn functionT(&self, num: u32, round: usize) -> u32 {
        let shifted = ((num << 8) | ((num & 0xff000000) >> 24)) as usize;
        let subbed =
            ((S[(shifted & 0x000000ff) >> (0 * 8)] as u32) << (0 * 8)) |
                ((S[(shifted & 0x0000ff00) >> (1 * 8)] as u32) << (0 * 8)) |
                ((S[(shifted & 0x00ff0000) >> (2 * 8)] as u32) << (0 * 8)) |
                ((S[(shifted & 0xff000000) >> (3 * 8)] as u32) << (0 * 8));
        subbed ^ RCON[round]
    }

    fn extend_key(&mut self) {
        let Nk = 4;
        let Nb = 4;
        let Nr = 10;
        let keys = Array::from(Vec::from(self.key.as_bytes()))
            .into_shape((4, self.key.bytes().len() / 4)).unwrap().fold_axis(Axis(1), 0, |a, b| (*a as u32) + (*b as u32));
        for i in 0..4 {
            self.w[[i]] = keys[[i]];
        }
        for i in Nk..(Nb * (Nr + 1)) {
            let temp =
                if i % Nk == 0 {
                    self.functionT(self.w[[i - 1]], i / Nk)
                } else {
                    self.w[[i - 1]]
                };
            self.w[[i]] = self.w[[i - Nk]] ^ temp;
        }
    }

    pub async fn encode(&mut self, reader: &mut dyn Read, writer: &mut dyn Write) {
        self.extend_key();
        loop {
            let mut source = [0 as u8; 16];
            let n = match reader.read(source.as_mut()) {
                Ok(n) => n,
                _ => 0
            };
            let done = n != 16;
            self.state = Array::<u8, _>::from(vec![source]).into_shape((4, 4)).unwrap();
            self.add_round_key(0);
            for i in 1..10 {
                self.sub_bytes();
                self.shift_rows();
                self.mix_columns();
                self.add_round_key(i);
            }
            self.sub_bytes();
            self.shift_rows();
            self.add_round_key(10);
            let mut data: [u8; 16] = [0; 16];
            let data_vec = self.state.clone().into_raw_vec();
            for (place, element) in data.iter_mut().zip(data_vec) {
                *place = element;
            }
            writer.write_all(&data).unwrap();
            if done { break; }
        }
        info!("done");
    }

    pub async fn decode(&mut self, reader: &mut dyn Read, writer: &mut dyn Write) {
        self.extend_key();
        loop {
            let mut source = [0 as u8; 16];
            let n = match reader.read(source.as_mut()) {
                Ok(n) => n,
                _ => 0
            };
            let done = n != 16;
            self.state = Array::<u8, _>::from(vec![source]).into_shape((4, 4)).unwrap();
            self.add_round_key(10);
            self.shift_rows();
            self.sub_bytes();
            for i in 1..10 {
                self.add_round_key(10 - i);
                self.mix_columns_inv();
                self.shift_rows_inv();
                self.sub_bytes_inv();
            }
            self.add_round_key(0);
            let mut data: [u8; 16] = [0; 16];
            let data_vec = self.state.clone().into_raw_vec();
            for (place, element) in data.iter_mut().zip(data_vec) {
                *place = element;
            }
            writer.write_all(&data).unwrap();
            if done { break; }
        }
        info!("done");
    }
}

// async fn run<T: Read>(stream: Bytes<T>, key: String, writer: &mut dyn Write) {
async fn run(reader: &mut dyn Read, writer: &mut dyn Write, key: &String, mode: RunMode, encode: bool) {
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
    block_on(run(&mut reader, &mut writer, &args.key, ECB, args.direction == "encode"));
}
