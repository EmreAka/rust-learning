#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::time::Instant;

const COUNT: usize = 4096;

fn main() {
    let mut data = vec![0.0f32; COUNT];
    let mut output = vec![0.0f32; data.len()];

    let mut rng = rand::thread_rng();
    for val in data.iter_mut() {
        *val = rng.gen();
    }

    let now = Instant::now();
    output.iter_mut().zip(data).for_each(|(value, input)| {
        *value = input;
        (0..3000000).into_iter().for_each(|_| {
            if *value > 0.5f32 {
                *value *= 0.99f32;
            } else {
                *value *= 1.01f32;
            }
        });
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {} seconds", elapsed.as_secs());
    println!("{}", output[23]);
}
