use super::utils::open_file;
use std::cmp::max;
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
    let mut count = 0i32;
    for line in buffer.lines() {
        let line_ref = line.as_ref().unwrap();
        let op = line_ref.chars().nth(0).unwrap() as i32 - 64;
        let player = line_ref.chars().nth(2).unwrap() as i32 - 87;
        let diff = player - op;
        if diff ==  0 {
            count += 3 + player;
        }
        else if diff < 0 {
            if diff.abs() == 1 {
                count += player;
            }
            else {
                count += 6 + player;
            }

        }
        else {
            if diff.abs() == 1{
                count += 6 + player;
            }
            else {
                count += player;
            }
        }
    }
    count as usize
}

fn part2(buffer: BufReader<File>) -> usize {
    let mut count = 0i32;
    for line in buffer.lines() {
        let line_ref = line.as_ref().unwrap();
        let op = line_ref.chars().nth(0).unwrap() as i32 - 64;
        let player = line_ref.chars().nth(2).unwrap() as i32 - 87;
        if player == 1{
            if op - 1 == 0{
                count += 3;
            }
            else {
                count += (op -1);
            }

        }
        else if player == 2 {
            count += op + 3;
        }
        else {
            if op == 3{
                count += 6 + 1;
            }
            else {
                count += 6 + op + 1
            }

        }
    }
    count as usize
}
