#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;



fn main() {
    //Unsigned integer : u8, u16, u32,u64, u128, usize
    //Signed integer : i8, i16, i32,i64, i128, isize
    let random_num:i32 = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);

    //using if 
    let mut my_age: i32 = 23;
    let can_vote= if my_age>= 18 {
        true 
    } else {
        false
    };

    println!("Can vote: {}", can_vote); 

    //using match
    let my_age2=8;
    match  my_age2{
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX=> println!("Impotant Birthday"),// will get everything greater than 65
        _=>println!("Not an important birthday"),
    };

    //checking voting age using match
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };

}

