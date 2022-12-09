mod days;

use days::*;
use paste::paste;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let day = 7;
    let path = format!("inputs/day_{}", day);
    println!("{}", day_7::run(&path, false));
}
