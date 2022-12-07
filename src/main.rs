mod days;

use days::*;
use paste::paste;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let day = 2;
    let path = format!("inputs/day_{}", day);
    println!("{}", day_2::run(&path, false));
    println!("{}", day_2::run(&path, true));
}
