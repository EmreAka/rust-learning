#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // let mut st1 = String::new();

    // st1.push('A');
    // st1.push_str(" WORD");

    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }

    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);


    let st1 = String::from("x r t b h k k a m c");
    //what is iterator and collector?
    let mut v1: Vec<char> = st1.chars().collect();

    v1.sort();
    v1.dedup();

    for char in v1 {
        println!("{}", char);
    }

    //String literal? not heap allocated!!
    let st2: &str = "Random string";
    //heap allocated!
    let mut st3: String = st2.to_string();
    println!("{}", st3);

    //byte array
    let byte_arr1 = st3.as_bytes();

    //Slice??
    let st4 = &st3[0..6];
    println!("String length: {}", st4.len());

    st3.clear();

    //doesnt exist after line 53
    let st5 = String::from("Just some");
    //exist after line 53
    let st6 = String::from(" Words.");
    let st7 = st5 + &st6;

    for char in st7.chars() {
        println!("{}", char);
    }

}
