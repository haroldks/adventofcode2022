use crate::days::utils::open_file;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Peekable;

pub fn run(input: &str, part_two: bool) -> usize {
    let buffer = open_file(input);
    match part_two {
        false => part1(buffer),
        true => part2(buffer),
    }
}

fn part1(buffer: BufReader<File>) -> usize {
    let mut score = 0usize;
    let mut folder_size = vec![];
    let mut input_iter = buffer.lines().into_iter().peekable();
    input_iter.next(); // Consume first line
    recursion(&mut input_iter, &mut score, &mut folder_size);
    score
}

fn part2(buffer: BufReader<File>) -> usize {
    let mut score = 0;
    let mut folder_size = vec![];
    let mut input_iter = buffer.lines().into_iter().peekable();
    input_iter.next(); // Consume first line
    let total = recursion(&mut input_iter, &mut score, &mut folder_size);
    folder_size.sort();
    let to_free = 30000000 - (70000000 - total);
    let to_remove = folder_size
        .iter()
        .filter(|val| **val >= to_free)
        .collect::<Vec<&usize>>()[0];
    *to_remove
}

fn recursion(
    lines_iterator: &mut Peekable<Lines<BufReader<File>>>,
    score: &mut usize,
    folder_size: &mut Vec<usize>,
) -> usize {
    lines_iterator.next(); // Consume ls
    let mut size = 0;
    while let Some(line) = lines_iterator.peek() {
        let line = line.as_ref().unwrap().clone(); // TODO: Check if I can use purely references
        if line.starts_with("dir") {
            lines_iterator.next();
            continue;
        }
        if line.starts_with("$") {
            break;
        }

        lines_iterator.next();
        let mut num = String::new();
        let mut chars = line.chars().collect::<Vec<char>>(); // FIXME: Useless but issue in indexing for chars
        for ch in chars {
            if ch.eq(&' ') {
                break;
            } else {
                num.push(ch);
            }
        }
        size += num.parse::<usize>().unwrap();
    }

    while let Some(line) = lines_iterator.next() {
        let line = line.unwrap();
        if line.ends_with("..") {
            break;
        }
        size += recursion(lines_iterator, score, folder_size);
    }

    if size <= 100000 {
        *score += size;
    }
    folder_size.push(size);
    return size;
}
