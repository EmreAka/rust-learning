#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let int_u8: u8 = 5;
    let int2_u8 = 4;

    //casting
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}
