#[cfg(test)]
mod tests {
    use ndarray::prelude::*;
    use ndarray::{array, concatenate, s, stack};

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
}