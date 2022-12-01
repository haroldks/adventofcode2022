mod days;

use days::*;
use paste::paste;
use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("{}", day_1::run("inputs/day_1", false));
    println!("{}", day_1::run("inputs/day_1", true));
}
