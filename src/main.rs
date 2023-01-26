#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    //Today is monday!!
    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("The worst day of the week"),
        Day::Tuesday => println!("EH"),
        Day::Wednesday => println!("Its good actually"),
        Day::Thursday => println!("Almost there!"),
        Day::Friday => println!("Finally!"),
        Day::Saturday => println!("WEEEKEND!!"),
        Day::Sunday => println!("End of the lovely weekend :("),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}
