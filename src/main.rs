#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let numbers = vec![1,2,3,4,5,6,7,8,10];

    let index = binary_search(8, numbers);
    println!("{}", index);
}

fn binary_search(search_number: i32, numbers: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = (numbers.len() - 1) as i32;

    while (left <= right) {
        let mid = (left + (right - left / 2)) as usize;

        if (search_number == numbers[mid]) {
            return mid as i32;
        }

        else if (search_number > numbers[mid]) {
            left = (mid + 1) as i32;
        }

        else {
            right = (mid - 1) as i32;
        }
    }

    return -1;
}
