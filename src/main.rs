#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What is your name?");
    //heap allocated!
    let mut name = String::new();
    let greeting = "Nice to meet you!";
    //recieve input and if cant get any input panic! with given message.
    io::stdin()
    .read_line(&mut name)
    .expect("Didn't receive input!");
    
    println!("Hello {}! {}", name.trim_end(), greeting)
}