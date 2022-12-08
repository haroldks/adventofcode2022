use crate::days::utils::open_file;
use std::collections::{HashMap, HashSet};
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
    let mut count: usize = 0;
    for line in buffer.lines() {
        let line = line.unwrap();
        let mut numbers = vec![];
        let mut number = String::new();
        for cha in line.chars() {
            if cha.eq(&'-') || cha.eq(&',') {
                numbers.push(number.parse::<usize>().unwrap());
                number = String::new();
            } else {
                number.push(cha);
            }
        }
        numbers.push(number.parse::<usize>().unwrap());
        if (numbers[0] == numbers[2])
            || (numbers[0] < numbers[2] && numbers[1] >= numbers[3])
            || (numbers[2] < numbers[0] && numbers[3] >= numbers[1])
        {
            count += 1;
        }
    }
    count
}

fn part2(buffer: BufReader<File>) -> usize {
    let mut count: usize = 0;
    for line in buffer.lines() {
        let line = line.unwrap();
        let mut numbers = vec![];
        let mut number = String::new();
        for cha in line.chars() {
            if cha.eq(&'-') || cha.eq(&',') {
                numbers.push(number.parse::<usize>().unwrap());
                number = String::new();
            } else {
                number.push(cha);
            }
        }
        numbers.push(number.parse::<usize>().unwrap());
        if (numbers[0] <= numbers[3] && numbers[1] >= numbers[2])
            || (numbers[2] <= numbers[1] && numbers[3] >= numbers[0])
        {
            count += 1;
        }
    }
    count
}
