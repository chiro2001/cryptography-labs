pub mod prime_gen {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::sync::mpsc;
    use std::thread;
    use chrono::Local;
    use lazy_static::lazy_static;
    use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
    use num_traits::*;
    use crate::rsa::config::config::*;
    use crate::rsa::prime_gen::prime_gen::PrimeError::Timeout;
    use mut_static::MutStatic;

    pub fn fast_modular_exponent(mut a: BigInt, mut q: BigInt, n: BigInt) -> BigInt {
        let mut r: BigInt = One::one();
        while q != Zero::zero() {
            if q.bit(0) { r = (r * &a) % &n; }
            q >>= 1;
            a = (&a * &a) % &n;
        }
        r
    }

    pub fn miller_rabin(n: &BigInt) -> Result<bool, Box<dyn Error>> {
        if n.is_zero() { return Ok(true); }
        if !n.bit(0) || n.is_one() { return Ok(false); }
        let mut rng = rand::thread_rng();
        let mut d: BigInt = n - 1.to_bigint().unwrap();
        while d.bit(0) { d >>= 1; }
        let tmp = d.clone();
        for _ in 0..CONFIG.read().unwrap().rounds {
            d = tmp.clone();
            let mut m = fast_modular_exponent(
                rng.gen_biguint_range(&Zero::zero(), &((n - 2.to_bigint().unwrap()).to_biguint().unwrap())).to_bigint().unwrap() + 2.to_bigint().unwrap(),
                d.clone(), n.clone());
            if m == One::one() { continue; } else {
                let mut pass = false;
                while d < *n {
                    if m == n - 1.to_bigint().unwrap() {
                        pass = true;
                        break;
                    }
                    m = (&m * &m) % n;
                    d <<= 1;
                }
                if !pass { return Ok(false); }
            }
        }
        Ok(true)
    }

    pub enum PrimeError {
        Timeout(u32)
    }

    impl PrimeError {
        fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Timeout(time) => write!(f, "Generation timeout after {} ms", time)
            }
        }
    }

    impl Display for PrimeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.display(f)
        }
    }

    impl Debug for PrimeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.display(f)
        }
    }

    impl Error for PrimeError {}

    lazy_static! {
        pub static ref PRIMES_CACHE: MutStatic<Vec<BigInt>> = MutStatic::from(Vec::new());
    }

    pub fn generate(low: &BigUint, high: &BigUint) -> Result<BigInt, PrimeError> {
        if !PRIMES_CACHE.read().unwrap().is_empty() {
            let prime = PRIMES_CACHE.write().unwrap().pop().unwrap().clone();
            if !CONFIG.read().unwrap().silent { println!("Use cached prime: {}", prime); }
            return Ok(prime);
        }
        let t = CONFIG.read().unwrap().threads;
        let l = (0..t).map(|_x| low.clone());
        let h = (0..t).map(|_x| high.clone());
        let (tx, rx) = mpsc::channel();
        let handles = l.zip(h).map(|x| {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(generate_one(&x.0, &x.1)).unwrap();
            })
        }).collect::<Vec<_>>();
        for _ in 0..t {
            match rx.recv().unwrap() {
                Ok(r) => PRIMES_CACHE.write().unwrap().push(r),
                _ => {}
            }
        }
        for handle in handles { handle.join().unwrap(); }
        if PRIMES_CACHE.read().unwrap().is_empty() {
            Err(PrimeError::Timeout(CONFIG.read().unwrap().time_max))
        } else {
            Ok(PRIMES_CACHE.write().unwrap().pop().unwrap())
        }
    }

    fn generate_one(low: &BigUint, high: &BigUint) -> Result<BigInt, PrimeError> {
        let mut rng = rand::thread_rng();
        let epoch = 0xf;
        let start = Local::now().timestamp_millis();
        let mut try_times = 0;
        loop {
            try_times += epoch;
            for _ in 0..epoch {
                let test = rng.gen_biguint_range(&low, &high).to_bigint().unwrap();
                if miller_rabin(&test).unwrap() {
                    let now = Local::now().timestamp_millis();
                    let time = now - start;
                    if !CONFIG.read().unwrap().silent {
                        println!("Done generation in {} tries after {} ms", try_times, time);
                    }
                    return Ok(test);
                }
            }
            let now = Local::now().timestamp_millis();
            let time = now - start;
            // assert!(time <= CONFIG.read().unwrap().time_max as i64);
            if time > CONFIG.read().unwrap().time_max as i64 {
                if !CONFIG.read().unwrap().silent {
                    println!("Failed generation in {} tries after {} ms", try_times, time);
                }
                return Err(PrimeError::Timeout(time as u32));
            }
        }
    }
}