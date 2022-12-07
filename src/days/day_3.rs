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
    let mut priority = 0;
    for line in buffer.lines() {
        let line = line.unwrap();
        let size = line.len();
        let mut char_map = HashMap::new();
        for (i, c) in line.chars().enumerate() {
            if i < size / 2 {
                char_map.insert(c, true);
            } else if char_map.contains_key(&c) {
                match c.is_lowercase() {
                    true => priority += c as usize - 96,
                    false => priority += c as usize - 38,
                };
                break;
            }
        }
    }
    priority
}

fn part2(buffer: BufReader<File>) -> usize {
    let mut priority = 0;
    let mut char_set: HashSet<char> = HashSet::new();
    for (id, line) in buffer.lines().enumerate() {
        let line = line.unwrap();
        if id % 3 == 0 {
            char_set = HashSet::new();
            for ch in line.chars() {
                char_set.insert(ch);
            }
        } else if id % 3 == 1 {
            let mut to_remove = vec![];
            for ch in char_set.iter() {
                if !line.contains(*ch) {
                    to_remove.push(*ch);
                }
            }
            for ch in to_remove {
                char_set.remove(&ch);
            }
        } else if id % 3 == 2 {
            for ch in line.chars() {
                if char_set.contains(&ch) {
                    match ch.is_lowercase() {
                        true => priority += ch as usize - 96,
                        false => priority += ch as usize - 38,
                    };
                    break;
                }
            }
        }
    }
    priority
}
