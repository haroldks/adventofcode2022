mod days;

use days::*;
use paste::paste;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let day = 4;
    let path = format!("inputs/day_{}", day);
    println!("{}", day_4::run(&path, true));
}
