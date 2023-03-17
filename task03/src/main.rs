#![feature(drain_filter)]
extern crate core;

use std::iter;
use std::iter::Iterator;
use std::fs;

//Try out array instead of named tuple, for fun
#[derive(Debug, Copy, Clone)]
struct Count(u32, u32);

impl Default for Count {
    fn default() -> Self {
        Count(0, 0)
    }
}

impl Count {
    fn add(&mut self, input: &char) -> () {
        match input {
            '0' => self.0 += 1,
            '1' => self.1 += 1,
            _ => ()
        }
    }

    fn most(&self) -> char {
        if self.0 > self.1 { '0' } else { '1' }
    }

    fn least(&self) -> char {
        if self.most() == '1' { '0' } else {'1'}
    }

    fn count_and_filter(mut lines: Vec<&str>, position: usize, selector: impl Fn(&Count) -> char) -> Vec<&str> {
        let count = lines.iter()
            .fold(Count::default(), |mut count, line| {
                count.add(&line.chars().nth(position).unwrap());
                count
            });
        lines.drain_filter(|line| line.chars().nth(position).unwrap() == selector(&count))
            .collect()
    }
}

fn binary_string_to_number(str: &str) -> u32 {
    u32::from_str_radix(str, 2).unwrap()
}

fn main() {
    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let test = false;
    let input = if test { String::from(TEST_INPUT) } else {
        fs::read_to_string("./input").expect("File should be there")
    };
    let line_length = input.find('\n').expect("Multiple lines expected");

    let init_counts: Vec<Count> = iter::repeat(Count::default())
        .take(line_length)
        .collect();
    let counts = input.lines()
        .fold(init_counts, |mut counts, line| {
            counts
                .iter_mut()
                .zip(line.chars())
                .for_each(|(count, char)| {
                    count.add(&char)
                });
            counts
        });
    let strings = counts
        .iter()
        .fold((String::default(), String::default()), |mut strings, count| {
            strings.0.push(count.most());
            strings.1.push(count.least());
            strings
        });
    let part1 = (
        binary_string_to_number(&strings.0),
        binary_string_to_number(&strings.1)
    );

    println!("Part 1: {}", part1.0 * part1.1);
    let mut most: Vec<&str> = input.lines().collect();
    let mut least: Vec<&str> = input.lines().collect();
    for position in 0..line_length {
        if most.len() > 1 {
            most = Count::count_and_filter(most, position, Count::most);
        }
        if least.len() > 1 {
            least = Count::count_and_filter(least, position, Count::least);
        }
    }

    let part2 = (
        binary_string_to_number(most.first().unwrap()),
        binary_string_to_number(least.first().unwrap())
    );

    println!("Part 2: {}", part2.0 * part2.1);
}
