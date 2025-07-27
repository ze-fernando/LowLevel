use crate::options::Options;
use rand::{Rng, thread_rng};

pub fn generate(config: Options, size: i32) -> String {
    let lower = "abcdefghijklmnopqrstuvwxyz";
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{}|;:',.<>?/\\\"`~";

    let mut hash = String::new();

    if config.lowercase {
        hash += lower;
    }
    if config.uppercase {
        hash += upper;
    }
    if config.number {
        hash += numbers;
    }
    if config.symbols {
        hash += symbols;
    }

    let mut rng = thread_rng();

    let password: String = (0..size)
        .map(|_| {
            let index = rng.gen_range(0..hash.len());
            hash.chars().nth(index).unwrap()
        })
        .collect();

    password
}
