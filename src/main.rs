mod days;

use days::*;
use paste::paste;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let day = 3;
    let path = format!("inputs/day_{}", day);
    println!("{}", day_3::run(&path, true));
}
