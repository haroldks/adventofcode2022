use super::utils::open_file;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(input: &str, part_two: bool) -> usize {
    let buffer = open_file(input);
    match part_two {
        false => {part1(buffer)}
        true => {part2(buffer)}
    }
}

fn get_elves_count(buffer: BufReader<File>) -> Vec<usize> {
    let mut elves_calories = vec![];
    let mut count: usize = 0;
    for line in buffer.lines() {
        let line_ref = line.as_ref().unwrap();
        if line_ref.len() == 0 {
            elves_calories.push(count);
            count = 0;
        } else {
            count += line_ref.parse::<usize>().unwrap();
        }
    }
    elves_calories
}

fn part1(buffer: BufReader<File>) -> usize {
    let mut elves_calories = get_elves_count(buffer);
    *elves_calories.iter().max().unwrap()
}

fn part2(buffer: BufReader<File>) -> usize {
    let mut elves_calories = get_elves_count(buffer);
    elves_calories.sort();
    let size = elves_calories.len();
    elves_calories[(size-3)..size].iter().sum::<usize>()
}

