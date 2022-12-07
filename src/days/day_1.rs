use super::utils::open_file;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(input: &str, part_two: bool) -> usize {
    let buffer = open_file(input);
    match part_two {
        false => part1(buffer),
        true => part2(buffer),
    }
}

fn part1(buffer: BufReader<File>) -> usize {
    let mut max_cal = 0usize;
    let mut count = 0usize;
    for line in buffer.lines() {
        let line_ref = line.as_ref().unwrap();
        if line_ref.len() == 0 {
            if count > max_cal {
                max_cal = count
            }
            count = 0;
        } else {
            count += line_ref.parse::<usize>().unwrap();
        }
    }
    max_cal
}

fn part2(buffer: BufReader<File>) -> usize {
    let mut max_1 = 0usize;
    let mut max_2 = 0usize;
    let mut max_3 = 0usize;
    let mut count = 0;
    for line in buffer.lines() {
        let line_ref = line.as_ref().unwrap();
        if line_ref.len() == 0 {
            if count > max_1 {
                max_3 = max_2;
                max_2 = max_1;
                max_1 = count
            } else if count > max_2 {
                max_3 = max_2;
                max_2 = count;
            } else if count > max_3 {
                max_3 = count;
            }
            count = 0;
        } else {
            count += line_ref.parse::<usize>().unwrap();
        }
    }
    max_3 + max_2 + max_1
}
