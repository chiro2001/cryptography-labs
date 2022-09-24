use std::fs::File;
use ndarray::prelude::*;
use ndarray::Array;
use futures::executor::block_on;
use std::io;
use std::io::{Bytes, Read, Stdin};

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
    fn encode() {

    }
}

async fn run<T: Read>(stream: Bytes<T>) {
    for b in stream {
        print!("{}", b.unwrap() as char);
    }
}

fn main() {
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

    // block_on(run(io::stdin().bytes()));
    block_on(run(File::open("xmake.lua").unwrap().bytes()));
}
