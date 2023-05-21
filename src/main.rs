#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut v: Vec<User> = Vec::new();
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("someusername1234"),
        email: String::from("someone1@example.com"),
        sign_in_count: 1,
    };

    v.push(user1);
    v.push(user2);

    for &user in v {
        println!("{0}", user.username)
    }

    for user in v {
        println!("{0}", user.username)
    }
}
