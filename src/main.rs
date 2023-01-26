#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_00;
    const PI: f32 = 3.141592;

    //shadowing -> same names dif data types
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number!");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
